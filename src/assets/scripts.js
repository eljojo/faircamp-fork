const failedIcon = document.querySelector('#failed_icon');
const loadingIcon = document.querySelector('#loading_icon');
const pauseIcon = document.querySelector('#pause_icon');
const playIcon = document.querySelector('#play_icon');
const successIcon = document.querySelector('#success_icon');

const bigPlaybackButton = document.querySelector('.big_play_button');

const copyFeedbackTimeouts = {};

window.activeTrack = null;

let globalOnPause = null;
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

async function mountAndPlay(container, seek) {
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

    if (audio.readyState === audio.HAVE_NOTHING) {
        container.classList.add('active');
        bigPlaybackButton.replaceChildren(loadingIcon.content.cloneNode(true));
        playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));

        window.activeTrack = track;

        audio.load();

        const loading = {};

        if (seek) {
            loading.seek = seek;
            seek = null;
        } else {
            loading.seek = null;
        }

        const aborted = await new Promise(resolve => {
            function tryFinishLoading() {
                if (audio.readyState < audio.HAVE_METADATA) return;

                if (loading.seek !== null) {
                    audio.currentTime = loading.seek;
                    // If currentTime is within 1ms of our requested seek time we consider
                    // the two equal (this accounts for float inaccuracies).
                    if (Math.abs(audio.currentTime - loading.seek) > 0.001) return;
                    loading.seek = null;
                }

                if (audio.readyState >= audio.HAVE_ENOUGH_DATA) {
                    delete track.loading;
                    clearInterval(loadInterval);
                    resolve(false);
                }
            }

            const loadInterval = setInterval(tryFinishLoading, 30);

            loading.abortLoading = () => {
                clearInterval(loadInterval);
                delete track.loading;
                container.classList.remove('active');
                bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
                playbackButton.replaceChildren(playIcon.content.cloneNode(true));
                resolve(true);
            };

            // We expose both `abortLoading` and `seek` on this loading object,
            // so that consecutive parallel playback requests may either abort
            // loading or reconfigure up to which time loading should occur (seek).
            track.loading = loading;
        });

        if (aborted) return;
    }

    if (!audio.dataset.boundListeners) {
        audio.dataset.boundListeners = true;

        audio.addEventListener('pause', event => {
            container.classList.remove('playing');
            bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
            playbackButton.replaceChildren(playIcon.content.cloneNode(true));
            clearInterval(globalUpdatePlayHeadInterval);

            if (globalOnPause !== null) {
                globalOnPause();
                globalOnPause = null;
            } else {
                updatePlayhead(track);
                announcePlayhead(waveformInput);
            }
        });

        audio.addEventListener('play', event => {
            container.classList.add('active', 'playing');
            bigPlaybackButton.replaceChildren(pauseIcon.content.cloneNode(true));
            playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));
            globalUpdatePlayHeadInterval = setInterval(() => updatePlayhead(track), 200);
            updatePlayhead(track);
            announcePlayhead(waveformInput);
        });

        audio.addEventListener('ended', event => {
            audio.currentTime = 0;
            container.classList.remove('active', 'playing');

            const nextContainer = container.nextElementSibling;
            if (nextContainer && nextContainer.classList.contains('track')) {
                togglePlayback(nextContainer);
            } else {
                window.activeTrack = null;
            }
        });
    }

    if (seek) { audio.currentTime = seek; }

    window.activeTrack = track;

    audio.play();
}

function togglePlayback(container = null, seek = null) {
    const { activeTrack } = window;

    if (activeTrack) {
        if (container === null) {
            container = activeTrack.container;
        }

        if (container === activeTrack.container) {
            // Here we do something with the already active track

            if (activeTrack.loading) {
                // This track is already requested for playback and actively loading.
                // We only update the requested `seek` if necessary.
                if (seek !== null) { activeTrack.loading.seek = seek; }
            } else if (activeTrack.audio.paused) {
                // This track is paused, we resume playback (and if requested, perform a seek beforehand)
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek;
                }
                activeTrack.audio.play();
            } else {
                // This track is playing, we either pause it, or perform a seek
                if (seek === null) {
                    activeTrack.audio.pause();
                } else {
                    activeTrack.audio.currentTime = seek;
                    updatePlayhead(activeTrack);
                    announcePlayhead(activeTrack.waveformInput);
                }
            }
        } else {
            // Another track is active, so we either abort its loading (if applies) or
            // pause it (if necessary) and reset it. Then we start the new track.

            if (activeTrack.loading) {
                activeTrack.loading.abortLoading();
                mountAndPlay(container, seek);
            } else {
                const resetCurrentStartNext = () => {
                    activeTrack.audio.currentTime = 0;
                    updatePlayhead(activeTrack, true);
                    announcePlayhead(activeTrack.waveformInput);
                    activeTrack.container.classList.remove('active');

                    mountAndPlay(container, seek);
                }

                if (activeTrack.audio.paused) {
                    resetCurrentStartNext();
                } else {
                    // The pause event occurs with a delay, so we defer resetting the track
                    // and starting the next one until just after the pause event fires.
                    globalOnPause = resetCurrentStartNext;
                    activeTrack.audio.pause();
                }

            }
        }
    } else {
        // No track is active, so we start either the requested one, or the first one on the page.

        if (container === null) {
            container = document.querySelector('.track');
        }

        mountAndPlay(container, seek);
    }
}

// While the underlying data model of the playhead (technically the invisible
// range input and visible svg representation) change granularly, we only
// trigger screenreader announcements when it makes sense - e.g. when
// focusing the range input, when seeking, when playback ends etc.
function announcePlayhead(waveformInput) {
    // TODO: Announce "current: xxxx, remaining: xxxxx"?
    waveformInput.setAttribute('aria-valuetext', formatTime(waveformInput.value));
}

function updatePlayhead(activeTrack, reset = false) {
    const { audio, time, waveformInput, waveformSvg } = activeTrack;
    const duration = parseFloat(waveformInput.max);
    const factor = reset ? 0 : audio.currentTime / duration;

    waveformSvg.querySelector('linearGradient.playback stop:nth-child(1)').setAttribute('offset', factor);
    waveformSvg.querySelector('linearGradient.playback stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    time.innerHTML = reset ? formatTime(duration) : `- ${formatTime(duration - audio.currentTime)}`;

    waveformInput.value = audio.currentTime;
}

if (bigPlaybackButton) {
    bigPlaybackButton.addEventListener('click', () => {
        togglePlayback();
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

for (const track of document.querySelectorAll('.track')) {
    const more = track.querySelector('.more');
    const moreButton = track.querySelector('.more_button');
    const playbackButton = track.querySelector('.track_playback');
    const waveform = track.querySelector('.waveform');
    const waveformInput = waveform.querySelector('input');

    playbackButton.addEventListener('click', event => {
        event.preventDefault();
        togglePlayback(track);
    });

    track.addEventListener('keydown', event => {
        if (event.key == ' ' || event.key == 'Enter') {
            event.preventDefault();
            togglePlayback(track);
        } else if (event.key == 'ArrowLeft') {
            event.preventDefault();
            const seek = Math.max(0, parseFloat(waveformInput.value) - 5);
            togglePlayback(track, seek);
        } else if (event.key == 'ArrowRight') {
            event.preventDefault();
            const seek = Math.min(parseFloat(waveformInput.max) - 1, parseFloat(waveformInput.value) + 5);
            togglePlayback(track, seek);
        }
    });

    waveform.addEventListener('click', event => {
        const factor = (event.clientX - waveformInput.getBoundingClientRect().x) / waveformInput.getBoundingClientRect().width;
        const seek = factor * waveformInput.max
        togglePlayback(track, seek);
        waveformInput.classList.add('focus_from_click');
        waveformInput.focus();
    });

    waveform.addEventListener('mouseenter', event => {
        track.classList.add('seek');
    });

    waveform.addEventListener('mousemove', event => {
        const factor = (event.clientX - waveform.getBoundingClientRect().x) / waveform.getBoundingClientRect().width;
        const waveformSvg = track.querySelector('.waveform svg');
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(1)').setAttribute('offset', factor);
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    });

    waveform.addEventListener('mouseout', event => {
        track.classList.remove('seek');
    });

    waveformInput.addEventListener('blur', () => {
        waveformInput.classList.remove('focus_from_click');
    });

    waveformInput.addEventListener('focus', () => {
        announcePlayhead(waveformInput);
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

