// TODO: Revisit this at some point - assumes we only have one script, and
// generally a bit hacky/unclean in nature.
// Also the alt/text and markup for the play/pause icon could drift away
// from how we write/implement it in our statically generated code.
const rootPrefix = document.querySelector('script').getAttribute('src').replace('scripts.js', '');
const pauseIcon = `<img alt="Pause" src="${rootPrefix}pause.svg" style="max-width: 1em;">`;
const playIcon = `<img alt="Play" src="${rootPrefix}play.svg" style="max-width: 1em;">`;

window.activeTrack = null;

const bigPlayButton = document.querySelector('.big_play_button');

function formatTime(seconds) {
    if (seconds < 60) {
        return `0:${Math.floor(seconds).toString().padStart(2, '0')}`;
    } else if (seconds < 3600) {
        return `${Math.floor(seconds / 60)}:${Math.floor(seconds % 60).toString().padStart(2, '0')}`;
    } else {
        return `${Math.floor(seconds % 3600)}:${Math.floor((seconds % 3600) / 60)}:${Math.floor(seconds % 60).toString().padStart(2, '0')}`;
    }
}

function mountAndPlay(container, seek) {
    const a = container.querySelector('a');
    const audio = container.querySelector('audio');
    const controlsInner = container.querySelector('.track_controls.inner');
    const controlsOuter = container.querySelector('.track_controls.outer');
    const svg = container.querySelector('svg');
    const time = container.querySelector('.track_time');

    if (!audio.dataset.endedListenerAdded) {
        audio.dataset.endedListenerAdded = true;
        
        audio.addEventListener('ended', event => {
            if (window.activeTrack && window.activeTrack.audio === audio) {
                audio.currentTime = 0;
                clearInterval(window.activeTrack.interval);
                updatePlayhead(window.activeTrack, true);
                container.classList.remove('active', 'playing');
                bigPlayButton.innerHTML = playIcon;
                controlsInner.innerHTML = playIcon;
                controlsOuter.innerHTML = playIcon;
                
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
    bigPlayButton.innerHTML = pauseIcon;
    controlsInner.innerHTML = pauseIcon;
    controlsOuter.innerHTML = pauseIcon;

    if (seek) {
        audio.currentTime = seek * audio.duration;
    }

    audio.play();

    window.activeTrack = { a, audio, container, controlsInner, controlsOuter, svg, time }
    window.activeTrack.interval = setInterval(() => updatePlayhead(window.activeTrack), 200);
}

function togglePlayback(container = null, seek = null) {
    const { activeTrack } = window;

    if (activeTrack) {
        if (container === null) {
            container = activeTrack.container;
        }

        if (container === activeTrack.container) {
            if (activeTrack.audio.paused) {
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek * activeTrack.audio.duration;
                }
                activeTrack.container.classList.add('playing');
                activeTrack.controlsInner.innerHTML = pauseIcon;
                activeTrack.controlsOuter.innerHTML = pauseIcon;
                bigPlayButton.innerHTML = pauseIcon;
                activeTrack.audio.play();
                activeTrack.interval = setInterval(() => updatePlayhead(activeTrack), 200);
            } else {
                if (seek !== null) {
                    activeTrack.audio.currentTime = seek * activeTrack.audio.duration;
                    updatePlayhead(activeTrack);
                } else {
                    activeTrack.audio.pause();
                    clearInterval(activeTrack.interval);
                    updatePlayhead(activeTrack);
                    activeTrack.container.classList.remove('playing');
                    activeTrack.controlsInner.innerHTML = playIcon;
                    activeTrack.controlsOuter.innerHTML = playIcon;
                    bigPlayButton.innerHTML = playIcon;
                }
            }
        } else {
            activeTrack.audio.pause();
            clearInterval(activeTrack.interval);
            activeTrack.audio.currentTime = 0;
            updatePlayhead(activeTrack, true);
            activeTrack.container.classList.remove('active', 'playing');
            activeTrack.controlsInner.innerHTML = playIcon;
            activeTrack.controlsOuter.innerHTML = playIcon;
            bigPlayButton.innerHTML = playIcon;

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
    const { audio, svg, time } = activeTrack;
    const factor = reset ? 0 : audio.currentTime / audio.duration;
    svg.querySelector('stop:nth-child(1)').setAttribute('offset', factor);
    svg.querySelector('stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
    time.innerHTML = reset ? '' : `${formatTime(audio.currentTime)} / `;
}

document.body.addEventListener('click', event => {
    if (event.target.classList.contains('big_play_button')) {
        togglePlayback();
    } else if (event.target.classList.contains('track_controls')) {
        event.preventDefault();
        const container = event.target.closest('.track')
        togglePlayback(container);
    } else if (event.target.classList.contains('track_title')) {
        event.preventDefault();
        const container = event.target.closest('.track')
        togglePlayback(container, 0);
    } else if (event.target.classList.contains('waveform')) {
        const container = event.target.closest('.track');
        const svg = event.target;
        const seek = (event.clientX - svg.getBoundingClientRect().x) / svg.getBoundingClientRect().width;
        togglePlayback(container, seek);
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

const MAX_TRACK_DURATION_WIDTH_EM = 20.0;
const TRACK_HEIGHT_EM = 1.5;
const WAVEFORM_PADDING_EM = 0.3;
const WAVEFORM_HEIGHT = TRACK_HEIGHT_EM - WAVEFORM_PADDING_EM * 2.0;

const waveformRenderState = {};

function waveforms(widthOverride) {
    let trackNumber = 1;
    for (const svg of document.querySelectorAll('svg[data-peaks]')) {
        const peaks = decode(svg.dataset.peaks).map(peak => peak / 63);

        const longestTrackDuration = parseInt(document.querySelector('[data-longest-duration]').dataset.longestDuration);
        const trackDuration = parseInt(svg.dataset.duration);

        let stepSize;
        let trackDurationWidthRem;
        if (widthOverride) {
            stepSize = 1; // TODO: Actually make this somewhat dependent on screen width (use less points when the screen is really small?)
            trackDurationWidthRem = widthOverride;
        } else {
            if (longestTrackDuration > 0) {
                trackDurationWidthRem = MAX_TRACK_DURATION_WIDTH_EM * (trackDuration / longestTrackDuration);
            } else {
                trackDurationWidthRem = 0; // TODO: Probably nonsensical (also by 0 division later on?), for now just copied from earlier rust implementation
            }
            stepSize = Math.floor(MAX_TRACK_DURATION_WIDTH_EM / trackDurationWidthRem);
        }

        let d = `M 0,${WAVEFORM_PADDING_EM + (1 - peaks[0]) * WAVEFORM_HEIGHT}`;

        const peakWidth = trackDurationWidthRem / peaks.length;

        for (let index = stepSize; index < peaks.length; index += stepSize) {
            const x = index * peakWidth;
            const y = WAVEFORM_PADDING_EM + (1 - peaks[index]) * WAVEFORM_HEIGHT;

            d += ` L ${x},${y}`;
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

        svg.setAttribute('viewBox', `0 0 ${trackDurationWidthRem} 1.5`);
        svg.setAttribute('width', `${trackDurationWidthRem}em`);
        svg.querySelector('.base').setAttribute('d', d);
        svg.querySelector('.progress').setAttribute('d', d);

        trackNumber++;
    }

    waveformRenderState.initialized = true;
}

waveforms();

window.addEventListener('resize', event => {
    const baseFontSizePx = parseInt(window
        .getComputedStyle(document.documentElement)
        .getPropertyValue('font-size')
        .replace('px', ''));
    const width = window.innerWidth;

    if (width > 350) {
        if (waveformRenderState.width !== Infinity) {
            waveforms();
            waveformRenderState.width = Infinity;
        }
    } else {
        if (waveformRenderState.width !== width) {
            const PADDING_HORIZONTAL = 2; // (rem) TODO: This is hardcoded, might change in css anytime - figure something out
            const widthRem = (1 / baseFontSizePx) * (width - PADDING_HORIZONTAL * baseFontSizePx);
            waveforms(widthRem);
            waveformRenderState.width = width;
        }
    }
})
