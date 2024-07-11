const loadingIcon = document.querySelector('#loading_icon');
const pauseIcon = document.querySelector('#pause_icon');
const playIcon = document.querySelector('#play_icon');

const bigPlaybackButton = document.querySelector('.big_play_button');

const copyNotificationTimeouts = {};

window.activeTrack = null;

function copyToClipboard(button) {
    const notify = (success, content) => {
        if (content in copyNotificationTimeouts) {
            clearTimeout(copyNotificationTimeouts[content]);
            delete copyNotificationTimeouts[content];
        }

        if (success) {
            const successIcon = button.querySelector('[data-icon="success"]');
            button.querySelector('.icon').replaceChildren(successIcon.content.cloneNode(true));
        } else {
            const failedIcon = button.querySelector('[data-icon="failed"]');
            button.querySelector('.icon').replaceChildren(failedIcon.content.cloneNode(true));
        }

        copyNotificationTimeouts[content] = setTimeout(() => {
            const copyIcon = button.querySelector('[data-icon="copy"]');
            button.querySelector('.icon').replaceChildren(copyIcon.content.cloneNode(true));
        }, 3000);
    };

    const content = button.dataset.content;
    navigator.clipboard
        .writeText(content)
        .then(() => notify(true, content))
        .catch(_err => notify(false, content));
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
    const a = container.querySelector('a');
    const audio = container.querySelector('audio');
    const playbackButton = container.querySelector('.track_playback');
    const time = container.querySelector('.duration');
    const waveformInput = container.querySelector('.waveform input');
    const waveformSvg = container.querySelector('.waveform svg');

    // The .duration property on the audio element is unreliable because during
    // loading it might be Infinity, or NaN, or only reflect the duration of
    // already loaded audio. So we also consider the duration we determined
    // when preprocessing the file.
    // TODO: We should store and make available the preprocessed track duration
    //       as float (right now it's int I believe), and additionally really
    //       guarantee that we *always* know the duration. Not being able to
    //       parse it should really be a hard error, this would make a lot of
    //       things easier to reason about and implement.
    // TODO: duration could/should probably come from the input range max,
    //       as a single source of truth, and to simplify data passing and
    //       calculations everywhere.
    const precalculatedDuration = parseFloat(waveformSvg.dataset.duration);
    const duration = () => {
        if (audio.duration === Infinity || audio.duration === NaN) {
            return precalculatedDuration;
        } else {
            return Math.max(audio.duration, precalculatedDuration);
        }
    };

    if (audio.readyState === audio.HAVE_NOTHING) {
        container.classList.add('active');
        playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));
        bigPlaybackButton.replaceChildren(loadingIcon.content.cloneNode(true));

        window.activeTrack = {
            a,
            audio,
            container,
            duration,
            playbackButton,
            time,
            waveformInput,
            waveformSvg
        };

        audio.load();

        const aborted = await new Promise(resolve => {
            const loadInterval = setInterval(() => {
                if (audio.readyState >= audio.HAVE_CURRENT_DATA) {
                    delete window.activeTrack.abortLoad;
                    clearInterval(loadInterval);
                    resolve(false);
                }
            }, 100);
            window.activeTrack.abortLoad = () => {
                delete window.activeTrack.abortLoad
                clearInterval(loadInterval);
                resolve(true);
            };
        });

        if (aborted) return;
    }

    if (!audio.dataset.endedListenerAdded) {
        audio.dataset.endedListenerAdded = true;
        
        audio.addEventListener('ended', event => {
            if (window.activeTrack && window.activeTrack.audio === audio) {
                audio.currentTime = 0;
                clearInterval(window.activeTrack.updatePlayHeadInterval);
                updatePlayhead(window.activeTrack, true);
                container.classList.remove('active', 'playing');
                bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
                playbackButton.replaceChildren(playIcon.content.cloneNode(true));
                
                const nextContainer = container.nextElementSibling;
                if (nextContainer && nextContainer.classList.contains('track')) {
                    togglePlayback(nextContainer);
                } else {
                    window.activeTrack = null;
                }
            }
        });
    }

    container.classList.add('active', 'playing');
    bigPlaybackButton.replaceChildren(pauseIcon.content.cloneNode(true));
    playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));

    if (seek) {
        audio.currentTime = seek;
    }

    audio.play();

    window.activeTrack = {
        a,
        audio,
        container,
        duration,
        playbackButton,
        time,
        waveformInput,
        waveformSvg
    };
    updatePlayhead(window.activeTrack);
    window.activeTrack.updatePlayHeadInterval = setInterval(
        () => updatePlayhead(window.activeTrack),
        200
    );
    announcePlayhead(waveformInput);
}

function togglePlayback(container = null, seek = null) {
    const { activeTrack } = window;

    if (activeTrack) {
        if (container === null) {
            container = activeTrack.container;
        }

        if (container === activeTrack.container) {
            // TODO: Here we are requesting to start playback on a track
            //       that is already loading because we previously requested
            //       to start its playback. For now we just drop this (new)
            //       playback request and wait for the previous one to go through.
            //       This should be improved though - on this (new) request we might
            //       have requested e.g. a specific seek position, this is currently
            //       discarded, but shouldn't be.
            if (activeTrack.abortLoad) return;

            if (activeTrack.audio.paused) {
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek;
                }
                activeTrack.container.classList.add('playing');
                bigPlaybackButton.replaceChildren(pauseIcon.content.cloneNode(true));
                activeTrack.playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));
                activeTrack.audio.play();
                activeTrack.updatePlayHeadInterval = setInterval(
                    () => updatePlayhead(activeTrack),
                    200
                );
                updatePlayhead(activeTrack);
                announcePlayhead(activeTrack.waveformInput);
            } else {
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek;
                    updatePlayhead(activeTrack);
                    announcePlayhead(activeTrack.waveformInput);
                } else {
                    activeTrack.audio.pause();
                    clearInterval(activeTrack.updatePlayHeadInterval);
                    updatePlayhead(activeTrack);
                    announcePlayhead(activeTrack.waveformInput);
                    activeTrack.container.classList.remove('playing');
                    bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
                    activeTrack.playbackButton.replaceChildren(playIcon.content.cloneNode(true));
                }
            }
        } else {
            if (activeTrack.abortLoad) activeTrack.abortLoad();

            activeTrack.audio.pause();
            clearInterval(activeTrack.updatePlayHeadInterval);
            activeTrack.audio.currentTime = 0;
            updatePlayhead(activeTrack, true);
            announcePlayhead(activeTrack.waveformInput);
            activeTrack.container.classList.remove('active', 'playing');
            bigPlaybackButton.replaceChildren(playIcon.content.cloneNode(true));
            activeTrack.playbackButton.replaceChildren(playIcon.content.cloneNode(true));

            mountAndPlay(container, seek);
        }
    } else {
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
    const { audio, duration, time, waveformInput, waveformSvg } = activeTrack;
    const factor = reset ? 0 : audio.currentTime / duration();

    waveformSvg.querySelector('linearGradient.playback stop:nth-child(1)').setAttribute('offset', factor);
    waveformSvg.querySelector('linearGradient.playback stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    time.innerHTML = reset ? formatTime(duration()) : `- ${formatTime(duration() - audio.currentTime)}`;

    waveformInput.value = audio.currentTime;
}

document.body.addEventListener('click', event => {
    const { target } = event;

    // TODO: Change to directing bindings, remove pointer-events: none on
    // buttons (respectively whereever else it would be prudent)
    if (target.classList.contains('track_playback')) {
        const container = target.closest('.track')
        togglePlayback(container);
    } else if ('copy' in target.dataset) {
        event.preventDefault();
        copyToClipboard(target);
    }
});

bigPlaybackButton.addEventListener('click', () => {
    togglePlayback();
});

for (const track of document.querySelectorAll('.track')) {
    const more = track.querySelector('.more');
    const moreButton = track.querySelector('.more_button');
    const waveformInput = track.querySelector('.waveform input');

    moreButton.addEventListener('focus', event => {
        // When this focus event occurs, the buttons in .more
        // just became visible and focusable, then we immediately
        // move focus to one of them, after which the .more_button
        // becomes invisible and unfocusable.
        more.querySelector(':first-child').focus();
        event.preventDefault();
    });

    waveformInput.addEventListener('change', () => {
        const container = waveformInput.closest('.track');
        const seek = waveformInput.value;
        togglePlayback(container, seek);
    });

    waveformInput.addEventListener('focus', () => {
        announcePlayhead(waveformInput);
    });

    waveformInput.addEventListener('keydown', event => {
        if (event.key == ' ' || event.key == 'Enter') {
            event.preventDefault();
            const container = waveformInput.closest('.track');
            togglePlayback(container);
        } else if (event.key == 'ArrowLeft') {
            const container = waveformInput.closest('.track');
            event.preventDefault();
            const seek = Math.max(0, parseFloat(waveformInput.value) - 5);
            togglePlayback(container, seek);
        } else if (event.key == 'ArrowRight') {
            const container = waveformInput.closest('.track');
            event.preventDefault();
            const seek = Math.min(parseFloat(waveformInput.max) - 1, parseFloat(waveformInput.value) + 5);
            togglePlayback(container, seek);
        }
    });

    waveformInput.addEventListener('mouseenter', event => {
        track.classList.add('seek');
    });

    waveformInput.addEventListener('mousemove', event => {
        const factor = (event.clientX - waveformInput.getBoundingClientRect().x) / waveformInput.getBoundingClientRect().width;
        const waveformSvg = track.querySelector('.waveform svg');
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(1)').setAttribute('offset', factor);
        waveformSvg.querySelector('linearGradient.seek stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    });

    waveformInput.addEventListener('mouseout', event => {
        track.classList.remove('seek');
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
    for (const svg of document.querySelectorAll('svg[data-peaks]')) {
        const peaks = decode(svg.dataset.peaks).map(peak => peak / 63);

        const trackDuration = parseFloat(svg.dataset.duration);

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
        for (button of document.querySelectorAll('[data-copy]')) {
            if (!button.dataset.content) {
                const thisPageUrl = window.location.href.split('#')[0]; // discard hash if present
                button.dataset.content = thisPageUrl;
            }
        }
    } else {
        for (button of document.querySelectorAll('[data-copy]')) {
            button.remove();
        }
    }
});

