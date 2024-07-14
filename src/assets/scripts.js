const failedIcon = document.querySelector('#failed_icon');
const loadingIcon = document.querySelector('#loading_icon');
const pauseIcon = document.querySelector('#pause_icon');
const playIcon = document.querySelector('#play_icon');
const successIcon = document.querySelector('#success_icon');

const bigPlaybackButton = document.querySelector('.big_play_button');

const copyFeedbackTimeouts = {};

let activeTrack = null;
let firstTrack = null;

let globalUpdatePlayHeadInterval;

function copyFeedback(content, feedbackIcon, iconContainer, originalIcon) {
    if (content in copyFeedbackTimeouts) {
        clearTimeout(copyFeedbackTimeouts[content]);
        delete copyFeedbackTimeouts[content];
    }

    iconContainer.replaceChildren(feedbackIcon.content.cloneNode(true));

    copyFeedbackTimeouts[content] = setTimeout(
        () => iconContainer.replaceChildren(originalIcon.content.cloneNode(true)),
        3000
    );
}

function copyToClipboard(button) {
    const content = button.dataset.content;
    const iconContainer = button.querySelector('.icon');
    const originalIcon = document.querySelector('#copy_icon');
    navigator.clipboard
        .writeText(content)
        .then(() => copyFeedback(content, successIcon, iconContainer, originalIcon))
        .catch(_err => copyFeedback(content, failedIcon, iconContainer, originalIcon));
};

function copyTrackToClipboard(button) {
    const content = button.dataset.content;
    const iconContainer = button;
    const originalIcon = document.querySelector('#copy_track_icon');
    navigator.clipboard
        .writeText(content)
        .then(() => copyFeedback(content, successIcon, iconContainer, originalIcon))
        .catch(_err => copyFeedback(content, failedIcon, iconContainer, originalIcon));
};

function formatTime(seconds) {
    if (seconds < 60) {
        return `0:${Math.floor(seconds).toString().padStart(2, '0')}`;
    } else {
        const secondsFormatted = Math.floor(seconds % 60).toString().padStart(2, '0');
        if (seconds < 3600) {
            return `${Math.floor(seconds / 60)}:${secondsFormatted}`;
        } else {
            return `${Math.floor(seconds / 3600)}:${Math.floor((seconds % 3600) / 60).toString().padStart(2, '0')}:${secondsFormatted}`;
        }
    }
}

async function mountAndPlay(track, seekTo) {
    activeTrack = track;

    // The pause and loading icon are visually indistinguishable (until the
    // actual loading animation kicks in after 500ms), hence we right away
    // transistion to the loading icon to make the interface feel snappy,
    // even if we potentially replace it with the pause icon right after that
    // if there doesn't end up to be any loading required.
    track.container.classList.add('active');
    track.playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));
    bigPlaybackButton.replaceChildren(loadingIcon.content.cloneNode(true));

    if (track.audio.preload !== 'auto') {
        track.audio.preload = 'auto';
        track.audio.load();
    }

    const play = () => {
        track.audio.play();
    };

    if (seekTo === null) {
        play();
    } else {
        const seeking = {
            to: seekTo
        };

        let closestPerformedSeek = 0;

        function tryFinishSeeking() {
            let closestAvailableSeek = 0;
            const { seekable } = track.audio;
            for (let index = 0; index < seekable.length; index++) {
                if (seekable.start(index) <= seeking.to) {
                    if (seekable.end(index) >= seeking.to) {
                        track.audio.currentTime = seeking.to;
                        delete track.seeking;
                        clearInterval(seekInterval);
                        play();
                    } else {
                        closestAvailableSeek = seekable.end(index);
                    }
                } else {
                    break;
                }
            }

            // If we can not yet seek to exactly the point we want to get to,
            // but we can get at least one second closer to that point, we do it.
            // (the idea being that this more likely triggers preloading of the
            // area that we need to seek to)
            if (seeking.to !== null && closestAvailableSeek - closestPerformedSeek > 1) {
                track.audio.currentTime = closestAvailableSeek;
                closestPerformedSeek = closestAvailableSeek;
            }
        }

        const seekInterval = setInterval(tryFinishSeeking, 30);

        seeking.abortSeeking = () => {
            clearInterval(seekInterval);
            delete track.seeking;
            track.container.classList.remove('active');
            track.playbackButton.replaceChildren(playIcon.content.cloneNode(true));
            bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
        };

        // We expose both `abortSeeking` and `seek` on this seeking object,
        // so that consecutive parallel playback requests may either abort
        // seeking or reconfigure up to which time seeking should occur (seek).
        track.seeking = seeking;
    }
}

function togglePlayback(track, seekTo = null) {
    if (!activeTrack) {
        mountAndPlay(track, seekTo);
    } else if (track === activeTrack) {
        if (track.seeking) {
            if (seekTo === null) {
                track.seeking.abortSeeking();
            } else {
                track.seeking.to = seekTo;
            }
        } else if (track.audio.paused) {
            if (seekTo !== null) {
                // TODO: Needs to be wrapped in an async mechanism that first ensures we can seek to that point
                track.audio.currentTime = seekTo;
            }
            track.audio.play();
        } else {
            // This track is playing, we either pause it, or perform a seek
            if (seekTo === null) {
                track.audio.pause();
            } else {
                // TODO: Needs to be wrapped in an async mechanism that first ensures we can seek to that point
                track.audio.currentTime = seekTo;
                updatePlayhead(track);
                announcePlayhead(track);
            }
        }
    } else {
        // Another track is active, so we either abort its loading (if applies) or
        // pause it (if necessary) and reset it. Then we start the new track.
        if (activeTrack.loading) {
            activeTrack.loading.abortSeeking();
            mountAndPlay(track, seekTo);
        } else {
            const resetCurrentStartNext = () => {
                activeTrack.audio.currentTime = 0;
                updatePlayhead(activeTrack, true);
                announcePlayhead(activeTrack);
                activeTrack.container.classList.remove('active');

                mountAndPlay(track, seekTo);
            }

            if (activeTrack.audio.paused) {
                resetCurrentStartNext();
            } else {
                // The pause event occurs with a delay, so we defer resetting the track
                // and starting the next one until just after the pause event fires.
                activeTrack.onPause = resetCurrentStartNext;
                activeTrack.audio.pause();
            }

        }
    }
}

// While the underlying data model of the playhead (technically the invisible
// range input and visible svg representation) change granularly, we only
// trigger screenreader announcements when it makes sense - e.g. when
// focusing the range input, when seeking, when playback ends etc.
function announcePlayhead(track) {
    const { waveformInput } = track;
    // TODO: Announce "current: xxxx, remaining: xxxxx"?
    waveformInput.setAttribute('aria-valuetext', formatTime(waveformInput.value));
}

function updatePlayhead(track, reset = false) {
    const { audio, time, waveformInput, waveformSvg } = track;
    const duration = parseFloat(waveformInput.max);
    const factor = reset ? 0 : audio.currentTime / duration;

    waveformSvg.querySelector('linearGradient.playback stop:nth-child(1)').setAttribute('offset', factor);
    waveformSvg.querySelector('linearGradient.playback stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    time.innerHTML = reset ? formatTime(duration) : `- ${formatTime(duration - audio.currentTime)}`;

    waveformInput.value = audio.currentTime;
}

if (bigPlaybackButton) {
    bigPlaybackButton.addEventListener('click', () => {
        togglePlayback(activeTrack ?? firstTrack);
    });
}

for (const copyButton of document.querySelectorAll('[data-copy]')) {
    copyButton.addEventListener('click', () => {
        copyToClipboard(copyButton);
    });
}

for (const copyTrackButton of document.querySelectorAll('[data-copy-track]')) {
    copyTrackButton.addEventListener('click', () => {
        copyTrackToClipboard(copyTrackButton);
    });
}

let previousTrack = null;
for (const container of document.querySelectorAll('.track')) {
    const audio = container.querySelector('audio');
    const playbackButton = container.querySelector('.track_playback');
    const time = container.querySelector('.duration');
    const waveformInput = container.querySelector('.waveform input');
    const waveformSvg = container.querySelector('.waveform svg');

    const track = {
        audio,
        container,
        playbackButton,
        time,
        waveformInput,
        waveformSvg
    };

    if (firstTrack === null) {
        firstTrack = track;
    }

    if (previousTrack !== null) {
        previousTrack.nextTrack = track;
    }

    previousTrack = track;

    audio.addEventListener('ended', event => {
        audio.currentTime = 0;
        container.classList.remove('active', 'playing');

        if (track.nextTrack) {
            togglePlayback(track.nextTrack);
        } else {
            activeTrack = null;
        }
    });

    audio.addEventListener('pause', event => {
        clearInterval(globalUpdatePlayHeadInterval);

        container.classList.remove('playing');
        playbackButton.replaceChildren(playIcon.content.cloneNode(true));
        bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));

        if (track.onPause) {
            track.onPause();
            delete track.onPause;
        } else {
            updatePlayhead(track);
            announcePlayhead(track);
        }
    });

    audio.addEventListener('play', event => {
        container.classList.add('active', 'playing');
        playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));
        bigPlaybackButton.replaceChildren(pauseIcon.content.cloneNode(true));
        globalUpdatePlayHeadInterval = setInterval(() => updatePlayhead(track), 200);
        updatePlayhead(track);
        announcePlayhead(track);
    });

    audio.addEventListener('playing', event => {
        playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));
        bigPlaybackButton.replaceChildren(pauseIcon.content.cloneNode(true));
    });

    audio.addEventListener('waiting', event => {
        // TODO: Eventually we could augment various screenreader labels here to
        //       indicate the loading state too
        playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));
        bigPlaybackButton.replaceChildren(loadingIcon.content.cloneNode(true));
    });

    playbackButton.addEventListener('click', event => {
        event.preventDefault();
        togglePlayback(track);
    });

    container.addEventListener('keydown', event => {
        if (event.key == 'ArrowLeft') {
            event.preventDefault();
            const seekTo = Math.max(0, parseFloat(waveformInput.value) - 5);
            togglePlayback(track, seekTo);
        } else if (event.key == 'ArrowRight') {
            event.preventDefault();
            const seekTo = Math.min(parseFloat(waveformInput.max) - 1, parseFloat(waveformInput.value) + 5);
            togglePlayback(track, seekTo);
        }
    });

    const waveform = container.querySelector('.waveform');

    waveform.addEventListener('click', event => {
        const factor = (event.clientX - waveformInput.getBoundingClientRect().x) / waveformInput.getBoundingClientRect().width;
        const seekTo = factor * waveformInput.max
        togglePlayback(track, seekTo);
        waveformInput.classList.add('focus_from_click');
        waveformInput.focus();
    });

    waveform.addEventListener('mouseenter', event => {
        container.classList.add('seek');
    });

    waveform.addEventListener('mousemove', event => {
        const factor = (event.clientX - waveform.getBoundingClientRect().x) / waveform.getBoundingClientRect().width;
        const waveformSvg = container.querySelector('.waveform svg');
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(1)').setAttribute('offset', factor);
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    });

    waveform.addEventListener('mouseout', event => {
        container.classList.remove('seek');
    });

    waveformInput.addEventListener('blur', () => {
        waveformInput.classList.remove('focus_from_click');
    });

    waveformInput.addEventListener('focus', () => {
        announcePlayhead(track);
    });

    waveformInput.addEventListener('keydown', event => {
        if (event.key == ' ' || event.key == 'Enter') {
            togglePlayback(track);
        }
    });
}

function decode(string) {
    const peaks = [];

    for (let index = 0; index < string.length; index++) {
        const code = string.charCodeAt(index);
        if (code >= 65 && code <= 90) { // A-Z
            peaks.push(code - 65); // 0-25
        } else if (code >= 97 && code <= 122) { // a-z
            peaks.push(code - 71); // 26-51
        } else if (code > 48 && code < 57) { // 0-9
            peaks.push(code + 4); // 52-61
        } else if (code === 43) { // +
            peaks.push(62);
        } else if (code === 48) { // /
            peaks.push(63);
        }
    }

    return peaks;
}

// IMPORTANT: Keep these three in sync with css
const PADDING_HORIZONTAL_REM = 2;
const BREAKPOINT_REDUCED_WAVEFORM_REM = 20;
const BREAKPOINT_MAX_WAVEFORM_REM = 30;

const MAX_TRACK_DURATION_WIDTH_EM = 20;
const REDUCED_TRACK_DURATION_WIDTH_EM = 18;
const TRACK_HEIGHT_EM = 1.5;
const WAVEFORM_PADDING_EM = 0.3;
const WAVEFORM_HEIGHT = TRACK_HEIGHT_EM - WAVEFORM_PADDING_EM * 2.0;

const waveformRenderState = {};

function waveforms() {
    const baseFontSizePx = parseFloat(
        window.getComputedStyle(document.documentElement)
              .getPropertyValue('font-size')
              .replace('px', '')
    );
    const viewportWidthRem = window.innerWidth / baseFontSizePx;

    let maxWaveformWidthRem;
    let relativeWaveforms;
    if (viewportWidthRem >= BREAKPOINT_MAX_WAVEFORM_REM) {
        maxWaveformWidthRem = MAX_TRACK_DURATION_WIDTH_EM;
        relativeWaveforms = !document.querySelector('[data-disable-relative-waveforms]');
    } else if (viewportWidthRem >= BREAKPOINT_REDUCED_WAVEFORM_REM) {
        maxWaveformWidthRem = REDUCED_TRACK_DURATION_WIDTH_EM;
        relativeWaveforms = !document.querySelector('[data-disable-relative-waveforms]');
    } else {
        maxWaveformWidthRem = viewportWidthRem - PADDING_HORIZONTAL_REM;
        relativeWaveforms = false;
    }

    if (waveformRenderState.widthRem === maxWaveformWidthRem) return;

    const longestTrackDuration = parseFloat(document.querySelector('[data-longest-duration]').dataset.longestDuration);

    let trackNumber = 1;
    for (const waveform of document.querySelectorAll('.waveform')) {
        const input = waveform.querySelector('input');
        const svg = waveform.querySelector('svg[data-peaks]');
        const peaks = decode(svg.dataset.peaks).map(peak => peak / 63);

        const trackDuration = parseFloat(input.max);

        let waveformWidthRem;
        if (longestTrackDuration > 0) {
            waveformWidthRem = maxWaveformWidthRem;

            if (relativeWaveforms) {
                waveformWidthRem *= (trackDuration / longestTrackDuration);
            }
        } else {
            // TODO: Probably nonsensical, (copied from earlier rust implementation)
            //       General topic/problem here is that we simply don't want a state
            //       where we have tracks whose length we don't know.
            waveformWidthRem = 0;
        }

        // Render the waveform with n samples. Prefer 0.75 samples per pixel, but if there
        // are less peaks available than that, sample exactly at every peak.
        // 1 samples per pixel = More detail, but more jagged
        // 0.5 samples per pixel = Smoother, but more sampling artifacts
        // 0.75 looked like a good in-between (on my low-dpi test screen anyway)
        const preferredNumSamples = Math.round(0.75 * waveformWidthRem * baseFontSizePx);
        const numSamples = Math.min(preferredNumSamples, peaks.length);

        const prevY = WAVEFORM_PADDING_EM + (1 - peaks[0]) * WAVEFORM_HEIGHT;
        let d = `M 0,${prevY.toFixed(2)}`;

        let yChangeOccured = false;
        for (let sample = 1; sample < numSamples; sample += 1) {
            const factor = sample / (numSamples - 1);
            const floatIndex = factor * (peaks.length - 1);
            const previousIndex = Math.floor(floatIndex);
            const nextIndex = Math.ceil(floatIndex);

            let peak;
            if (previousIndex === nextIndex) {
                peak = peaks[previousIndex];
            } else {
                const interPeakBias = floatIndex - previousIndex;
                peak = peaks[previousIndex] * (1 - interPeakBias) + peaks[nextIndex] * interPeakBias;
            }

            const x = factor * waveformWidthRem;
            const y = WAVEFORM_PADDING_EM + (1 - peak) * WAVEFORM_HEIGHT;

            // If the y coordinate is always exactly the same on all points, the linear
            // gradient applied to the .playback path does not show up at all (firefox).
            // This only happens when the track is perfectly silent/same level all the
            // way through, which currently is the case when with the disable_waveforms option.
            // We counter this here by introducing minimal jitter on the y dimension.
            const yJitter = (y === prevY ? '1' : '');

            d += ` L ${x.toFixed(2)},${y.toFixed(2)}${yJitter}`;
        }

        const SVG_XMLNS = 'http://www.w3.org/2000/svg';

        if (!waveformRenderState.initialized) {
            svg.setAttribute('xmlns', SVG_XMLNS);
            svg.setAttribute('height', `${TRACK_HEIGHT_EM}em`);

            const defs = document.createElementNS(SVG_XMLNS, 'defs');

            const playbackGradient = document.createElementNS(SVG_XMLNS, 'linearGradient');
            playbackGradient.classList.add('playback');
            playbackGradient.id = `gradient_playback_${trackNumber}`;
            const playbackGradientStop1 = document.createElementNS(SVG_XMLNS, 'stop');
            playbackGradientStop1.setAttribute('offset', '0');
            playbackGradientStop1.setAttribute('stop-color', 'var(--fg-1)');
            const playbackGradientStop2 = document.createElementNS(SVG_XMLNS, 'stop');
            playbackGradientStop2.setAttribute('offset', '0.000001');
            playbackGradientStop2.setAttribute('stop-color', 'hsla(0, 0%, 0%, 0)');
            playbackGradient.append(playbackGradientStop1, playbackGradientStop2);

            const seekGradient = document.createElementNS(SVG_XMLNS, 'linearGradient');
            seekGradient.classList.add('seek');
            seekGradient.id = `gradient_seek_${trackNumber}`;
            const seekGradientStop1 = document.createElementNS(SVG_XMLNS, 'stop');
            seekGradientStop1.setAttribute('offset', '0');
            seekGradientStop1.setAttribute('stop-color', 'var(--fg-3)');
            const seekGradientStop2 = document.createElementNS(SVG_XMLNS, 'stop');
            seekGradientStop2.setAttribute('offset', '0.000001');
            seekGradientStop2.setAttribute('stop-color', 'hsla(0, 0%, 0%, 0)');
            seekGradient.append(seekGradientStop1, seekGradientStop2);

            defs.append(playbackGradient);
            defs.append(seekGradient);
            svg.prepend(defs);

            svg.querySelector('path.playback').setAttribute('stroke', `url(#gradient_playback_${trackNumber})`);
            svg.querySelector('path.seek').setAttribute('stroke', `url(#gradient_seek_${trackNumber})`);
        }

        svg.setAttribute('viewBox', `0 0 ${waveformWidthRem} 1.5`);
        svg.setAttribute('width', `${waveformWidthRem}em`);
        svg.querySelector('path.base').setAttribute('d', d);
        svg.querySelector('path.playback').setAttribute('d', d);
        svg.querySelector('path.seek').setAttribute('d', d);

        trackNumber++;
    }

    waveformRenderState.initialized = true;
    waveformRenderState.widthRem = maxWaveformWidthRem;
}

window.addEventListener('DOMContentLoaded', event => {
    // TODO: Potentially split player js into seperate script file
    //       so we don't need the check, and only load the additional
    //       js payload where it's needed.
    if (document.querySelector('[data-peaks]')) {
        waveforms();
        window.addEventListener('resize', waveforms);
    }

    if (navigator.clipboard) {
        for (button of document.querySelectorAll('[data-copy], [data-copy-track]')) {
            if (button.dataset.dynamicUrl !== undefined) {
                if (button.dataset.dynamicUrl === "") {
                    // Build link to this page dynamically
                    const thisPageUrl = window.location.href.split('#')[0]; // discard hash if present
                    button.dataset.content = thisPageUrl;
                } else {
                    // Build link to subpage dynamically
                    let subPageUrl = window.location.href.split('#')[0]; // discard hash if present
                    if (!subPageUrl.endsWith('/')) { subPageUrl += '/' }
                    subPageUrl += button.dataset.dynamicUrl;
                    button.dataset.content = subPageUrl;
                }
            }
        }
    } else {
        for (button of document.querySelectorAll('[data-copy], [data-copy-track]')) {
            button.remove();
        }
    }
});
