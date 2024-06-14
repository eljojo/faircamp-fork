const loadingIcon = document.querySelector('#loading_icon');
const pauseIcon = document.querySelector('#pause_icon');
const playIcon = document.querySelector('#play_icon');

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
    const controlsInner = container.querySelector('.track_controls.inner');
    const controlsOuter = container.querySelector('.track_controls.outer');
    const svg = container.querySelector('.waveform');
    const time = container.querySelector('.track_time');

    // The .duration property on the audio element is unreliable because during
    // loading it might be Infinity, or NaN, or only reflect the duration of
    // already loaded audio. So we also consider the duration we determined
    // when preprocessing the file.
    // TODO: We should store and make available the preprocessed track duration
    //       as float (right now it's int I believe), and additionally really
    //       guarantee that we *always* know the duration. Not being able to
    //       parse it should really be a hard error, this would make a lot of
    //       things easier to reason about and implement.
    const precalculatedDuration = parseFloat(svg.dataset.duration);
    const duration = () => {
        if (audio.duration === Infinity || audio.duration === NaN) {
            return precalculatedDuration;
        } else {
            return Math.max(audio.duration, precalculatedDuration);
        }
    };

    if (audio.readyState === audio.HAVE_NOTHING) {
        container.classList.add('active');
        controlsInner.replaceChildren(loadingIcon.content.cloneNode(true));
        controlsOuter.replaceChildren(loadingIcon.content.cloneNode(true));

        window.activeTrack = {
            a,
            audio,
            container,
            controlsInner,
            controlsOuter,
            duration,
            svg,
            time
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
                controlsInner.replaceChildren(playIcon.content.cloneNode(true));
                controlsOuter.replaceChildren(playIcon.content.cloneNode(true));
                
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
    controlsInner.replaceChildren(pauseIcon.content.cloneNode(true));
    controlsOuter.replaceChildren(pauseIcon.content.cloneNode(true));

    if (seek) {
        audio.currentTime = seek * duration();
    }

    audio.play();

    window.activeTrack = {
        a,
        audio,
        container,
        controlsInner,
        controlsOuter,
        duration,
        svg,
        time
    };
    window.activeTrack.updatePlayHeadInterval = setInterval(
        () => updatePlayhead(window.activeTrack),
        200
    );
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
                    activeTrack.audio.currentTime = seek * activeTrack.duration();
                }
                activeTrack.container.classList.add('playing');
                activeTrack.controlsInner.replaceChildren(pauseIcon.content.cloneNode(true));
                activeTrack.controlsOuter.replaceChildren(pauseIcon.content.cloneNode(true));
                activeTrack.audio.play();
                activeTrack.updatePlayHeadInterval = setInterval(
                    () => updatePlayhead(activeTrack),
                    200
                );
            } else {
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek * activeTrack.duration();
                    updatePlayhead(activeTrack);
                } else {
                    activeTrack.audio.pause();
                    clearInterval(activeTrack.updatePlayHeadInterval);
                    updatePlayhead(activeTrack);
                    activeTrack.container.classList.remove('playing');
                    activeTrack.controlsInner.replaceChildren(playIcon.content.cloneNode(true));
                    activeTrack.controlsOuter.replaceChildren(playIcon.content.cloneNode(true));
                }
            }
        } else {
            if (activeTrack.abortLoad) activeTrack.abortLoad();

            activeTrack.audio.pause();
            clearInterval(activeTrack.updatePlayHeadInterval);
            activeTrack.audio.currentTime = 0;
            updatePlayhead(activeTrack, true);
            activeTrack.container.classList.remove('active', 'playing');
            activeTrack.controlsInner.replaceChildren(playIcon.content.cloneNode(true));
            activeTrack.controlsOuter.replaceChildren(playIcon.content.cloneNode(true));

            mountAndPlay(container, seek);
        }
    } else {
        if (container === null) {
            container = document.querySelector('.track');
        }

        mountAndPlay(container, seek);
    }
}

function updatePlayhead(activeTrack, reset = false) {
    const { audio, duration, svg, time } = activeTrack;
    const factor = reset ? 0 : audio.currentTime / duration();
    svg.querySelector('stop:nth-child(1)').setAttribute('offset', factor);
    svg.querySelector('stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    time.innerHTML = reset ? '' : `${formatTime(audio.currentTime)} / `;
}

document.body.addEventListener('click', event => {
    const { target } = event;

    if (target.classList.contains('big_play_button')) {
        togglePlayback();
    } else if (target.classList.contains('track_controls')) {
        event.preventDefault();
        const container = target.closest('.track')
        togglePlayback(container);
    } else if (target.classList.contains('track_title')) {
        event.preventDefault();
        const container = target.closest('.track')
        togglePlayback(container, 0);
    } else if (target.classList.contains('waveform')) {
        const container = target.closest('.track');
        const svg = target;
        const seek = (event.clientX - svg.getBoundingClientRect().x) / svg.getBoundingClientRect().width;
        togglePlayback(container, seek);
    } else if ('copy' in target.dataset) {
        event.preventDefault();
        copyToClipboard(target);
    }
});

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
            // gradient applied to the .progress path does not show up at all (firefox).
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
            const linearGradient = document.createElementNS(SVG_XMLNS, 'linearGradient');
            linearGradient.id = `gradient_${trackNumber}`;
            const stop1 = document.createElementNS(SVG_XMLNS, 'stop');
            stop1.setAttribute('offset', '0');
            stop1.setAttribute('stop-color', 'hsl(0, 0%, var(--text-l))');
            const stop2 = document.createElementNS(SVG_XMLNS, 'stop');
            stop2.setAttribute('offset', '0.000001');
            stop2.setAttribute('stop-color', 'hsla(0, 0%, 0%, 0)');

            linearGradient.append(stop1, stop2);
            defs.append(linearGradient);
            svg.prepend(defs);

            svg.querySelector('.progress').setAttribute('stroke', `url(#gradient_${trackNumber})`);
        }

        svg.setAttribute('viewBox', `0 0 ${waveformWidthRem} 1.5`);
        svg.setAttribute('width', `${waveformWidthRem}em`);
        svg.querySelector('.base').setAttribute('d', d);
        svg.querySelector('.progress').setAttribute('d', d);

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

