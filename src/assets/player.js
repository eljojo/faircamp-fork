// Constants used in drawing the waveform svgs
const BREAKPOINT_REDUCED_WAVEFORM_REM = 20;
const TRACK_HEIGHT_EM = 1.5;
const WAVEFORM_PADDING_EM = 0.3;
const WAVEFORM_HEIGHT = TRACK_HEIGHT_EM - WAVEFORM_PADDING_EM * 2.0;
const WAVEFORM_WIDTH_PADDING_REM = 5;
const WAVEFORM_WIDTH_TOLERANCE_REM = 2.5;

const loadingIcon = document.querySelector('#loading_icon').content;
const pauseIcon = document.querySelector('#pause_icon').content;
const playIcon = document.querySelector('#play_icon').content;

const listenButton = document.querySelector('button.listen');
const listenButtonIcon = document.querySelector('button.listen .icon');
const listenButtonLabel = document.querySelector('button.listen .label');

// There is always an active track (so this is guaranteed to be available
// after the respective intialization routines have completed). "active"
// refers to various states, but it generally indicates that the track is
// selected for being played back, open in the docked player, in the process
// of loading/seeking, or already being played back. Conversely, any not
// active track is guaranteed to be cleaned up in terms of its state, i.e.
// not being played back, not being displayed (in a place where it needs
// track-specific clean-up), not running asynchronous routines (that don't
// clean themselves up invisibly when they finish), etc. Differentiation of
// the exact state of an active track happens through properties that are set
// on it, see the respective parts in the code.
let activeTrack;

const tracks = [];

const dockedPlayerContainer = document.querySelector('.docked_player');
const dockedPlayer = {
    container: dockedPlayerContainer,
    currentTime: dockedPlayerContainer.querySelector('.time .current'),
    nextTrackButton: dockedPlayerContainer.querySelector('button.next_track'),
    number: dockedPlayerContainer.querySelector('.number'),
    playbackButton: dockedPlayerContainer.querySelector('button.playback'),
    progress: dockedPlayerContainer.querySelector('.progress'),
    speedButton: dockedPlayerContainer.querySelector('button.speed'),
    speedMultiplier: dockedPlayerContainer.querySelector('button.speed .multiplier'),
    status: document.querySelector('.docked_player_status'),
    timeline: dockedPlayerContainer.querySelector('.timeline'),
    timelineInput: dockedPlayerContainer.querySelector('.timeline input'),
    titleWrapper: dockedPlayerContainer.querySelector('.title_wrapper'),
    totalTime: dockedPlayerContainer.querySelector('.time .total'),
    volume: {
        container: dockedPlayerContainer.querySelector('.volume'),
        knob: dockedPlayerContainer.querySelector('.volume button'),
        knobSvgTitle: dockedPlayerContainer.querySelector('.volume button svg title'),
        slider: dockedPlayerContainer.querySelector('.volume .slider'),
        sliderInput: dockedPlayerContainer.querySelector('.volume .slider input'),
        sliderSvg: dockedPlayerContainer.querySelector('.volume .slider svg')
    }
};

let globalUpdatePlayHeadInterval;

// We internally manage speed as (integer) percent values to avoid having to
// deal with float rounding issues.
let speed = 100;

const volume = {
    finegrained: false,
    level: 1
};

// When a page loads we start with the assumption that the volume property on
// audio elements is read-only, but immediately run an asynchronous routine
// to determine if volume is actually mutable - if it is we register this on
// our global volume object and append a class to the volume controls
// container so that by the time the visitor initiates audio playback they
// can potentially interact with the volume controls on a fine-grained
// levels. The reason for this quirky stuff is that Apple's iOS devices
// intentionally don't allow application-level volume control and therefore
// the web audio API on these devices features a read-only volume property on
// audio elements.
let volumeProbe = new Audio();
const volumeProbeHandler = () => {
    dockedPlayer.volume.container.classList.add('finegrained');
    volume.finegrained = true;
    updateVolume(false);
    volumeProbe.removeEventListener('volumechange', volumeProbeHandler);
    volumeProbe = null;
};
volumeProbe.addEventListener('volumechange', volumeProbeHandler);
volumeProbe.volume = 0.123;

const persistedVolume = localStorage.getItem('faircampVolume');
if (persistedVolume !== null) {
    const level = parseFloat(persistedVolume);
    if (level >= 0 && level <= 1) {
        volume.level = level;
    }
}
updateVolume(false);

// While the underlying data model of the playhead (technically the invisible
// range input and visible svg representation) change granularly, we only
// trigger screenreader announcements when it makes sense - e.g. when
// focusing the range input, when seeking, when playback ends etc.
function announcePlayhead(track) {
    const valueText = `${PLAYER_JS_T.playbackPosition} ${formatTimeWrittenOut(dockedPlayer.timelineInput.value)}`;

    dockedPlayer.timelineInput.setAttribute('aria-valuetext', valueText);

    if (track.waveform) {
        track.waveform.input.setAttribute('aria-valuetext', valueText);
    }
}

// Decodes a sequence of peaks that is encoded using a custom base64 alphabet
// (A-Za-z0-9+/) into a sequence of numbers (0-63)
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

function formatTimeWrittenOut(seconds) {
    if (seconds < 60) {
        return PLAYER_JS_T.xxxSeconds(Math.floor(seconds));
    } else {
        const secondsWrittenOut = PLAYER_JS_T.xxxSeconds(Math.floor(Math.floor(seconds % 60)));
        if (seconds < 3600) {
            return `${PLAYER_JS_T.xxxMinutes(Math.floor(seconds / 60))} ${secondsWrittenOut}`;
        } else {
            return `${PLAYER_JS_T.xxxHours(Math.floor(seconds / 3600))} ${PLAYER_JS_T.xxxMinutes(Math.floor((seconds % 3600) / 60))} ${secondsWrittenOut}`;
        }
    }
}

// In most cases, hover capability remains constant during runtime, however
// during testing in desktop browsers with device simulation and possibly in
// some edge cases in real usage, hover capability might change at runtime,
// therefore we always query this capability dynamically.
function hoverAvailable() {
    return window.matchMedia('(hover: hover)').matches;
}

// Open the docked player and update its various subelements to display the
// given track. If track.seekTo is set a seek is indicated by advancing both
// the track's own progress indicator and the docked player progress bar to
// the seek point (that seek however isn't performed yet, that's only done
// when playback is initiated).
function open(track) {
    // Unhide docked player
    document.body.classList.add('player_active');

    dockedPlayer.currentTime.textContent = formatTime(track.seekTo ?? track.audio.currentTime);
    dockedPlayer.totalTime.textContent = formatTime(track.duration);
    dockedPlayer.timelineInput.max = track.container.dataset.duration;

    if (track.artists) {
        dockedPlayer.titleWrapper.replaceChildren(track.title.cloneNode(true), track.artists.cloneNode(true));
    } else {
        dockedPlayer.titleWrapper.replaceChildren(track.title.cloneNode(true));
    }

    // Not available on a track player
    if (dockedPlayer.number) {
        dockedPlayer.nextTrackButton.toggleAttribute('disabled', !track.nextTrack);
        dockedPlayer.number.textContent = track.number.textContent;
    }

    if (track.seekTo !== undefined && track.seekTo > 0) {
        const factor = track.seekTo / track.duration;

        dockedPlayer.progress.style.setProperty('width', `${factor * 100}%`);
        dockedPlayer.currentTime.textContent = formatTime(track.seekTo);
        dockedPlayer.timelineInput.value = track.seekTo;

        if (track.waveform) {
            track.waveform.svg.querySelector('linearGradient.playback stop:nth-child(1)').setAttribute('offset', factor);
            track.waveform.svg.querySelector('linearGradient.playback stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
            track.waveform.input.value = track.seekTo;
        }
    }

    track.open = true;
}

// Parses (and validates) track/time parameters from the current url
// (e.g. https://example.com/#track=3&time=4m12s) and returns them as an
// object (e.g. { time: 252, track: [reference to track] }). Track can be
// specified as n=1 or track=1, time can be specified as t=60 or time=60, but
// also supports complex specifiers like t=1h, t=1m t=1s, t=1h1m, t=1h1m1s,
// etc. In case of no known params being present or errors being encountered
// (wrong syntax for params, out-of-bound track numbers or timecodes, etc.)
// null is returned. Note that in case of a non-null return value, there is
// always a reference to a track returned (!), i.e. if the hash specified
// only #t=60, this is interpreted as "seek to 60 seconds on the first
// track".
function parseHashParams() {
    if (location.hash.length === 0) return null;

    const params = new URLSearchParams(location.hash.substring(1));

    const timeParam = params.get('t') ?? params.get('time');
    const trackParam = params.get('n') ?? params.get('track');

    if (timeParam === null && trackParam === null) return null;

    const result = {};

    if (trackParam === null) {
        result.track = tracks[0];
    } else if (trackParam.match(/^[0-9]+$/)) {
        const index = parseInt(trackParam) - 1;

        if (index < tracks.length) {
            result.track = tracks[index]
        }
    }

    if (!result.track) return null;

    if (timeParam !== null) {
        // Match all of "1", "1s", "1m" "1h" "1m1s", "1h1m1s", "1h1s", "1h1m", etc.
        const match = timeParam.match(/^(?:([0-9]+)h)?(?:([0-9]+)m)?(?:([0-9]+)s?)?$/);

        if (match) {
            result.time = 0;

            const [_, h, m, s] = match;

            if (h) { result.time += parseInt(h) * 3600; }
            if (m) { result.time += parseInt(m) * 60; }
            if (s) { result.time += parseInt(s); }

            if (result.time > result.track.duration) {
                return null;
            }
        } else {
            return null;
        }
    }

    return result;
}

function play(track) {
    if (!track.open) {
        open(track);

        // Announce to screenreaders that the docked player is present
        dockedPlayer.status.setAttribute('aria-label', PLAYER_JS_T.playerOpenPlayingXxx(track.title.textContent));
    }

    if (track.audio.preload !== 'auto') {
        track.audio.preload = 'auto';
        track.audio.load();
    }

    // The pause and loading icon are visually indistinguishable (until the
    // actual loading animation kicks in after 500ms), hence we right away
    // transistion to the loading icon to make the interface feel snappy,
    // even if we potentially replace it with the pause icon right after that
    // if there doesn't end up to be any loading required.
    track.playbackButtonIcon.replaceChildren(loadingIcon.cloneNode(true));
    dockedPlayer.playbackButton.replaceChildren(loadingIcon.cloneNode(true));
    listenButtonIcon.replaceChildren(loadingIcon.cloneNode(true));
    listenButtonLabel.textContent = PLAYER_JS_T.pause;

    const playCallback = () => {
        // On apple devices and browsers (e.g. Safari in macOS 15.1) there is
        // a bug where, when multiple tracks play back while another
        // application but Safari has focus, on returning focus to Safari,
        // multiple/all previously played tracks start playing at the same
        // time. Investigation indicated that these playback requests come
        // from the system/browser itself and that there was/is no faulty
        // asynchronous routine in faircamp's code found that causes this. In
        // order to work around this unexplained problem, we're tagging
        // tracks with a flag (track.solicitedPlayback) right before we
        // explicitly play them (this flag is unset again with each pause
        // event) and generally cancel playback requests on tracks where the
        // flag is not set - these we know to originate from the
        // system/browser.
        track.solicitedPlayback = true;
        setSpeed(track);
        setVolume(track);
        track.audio.play();
    }

    if (track.seekTo === undefined) {
        playCallback();
    } else {
        seek(track, playCallback);
    }
}

// One of the following:
// - Request to play the active track
// - Request to cancel seeking/loading the active track
// - Request to pause the active track
// - Request to reset the active track and play another
function requestPlaybackChange(track) {
    if (track === activeTrack) {
        if (track.seeking) {
            track.seeking.cancel();
        } else if (track.audio.paused) {
            play(track);
        } else {
            track.audio.pause();
        }
    } else {
        const playNext = () => {
            setActive(track);
            play(track);
        };

        reset(activeTrack, playNext);
    }
}

// One of the following:
// - Request to make the active and playing track jump to another point
// - Request to play the active but paused track from a specific point
// - Request to make the active but currently seeking/loading track seek to another point
// - Request to reset the active track and play another from a specific point
function requestSeek(track, seekTo) {
    if (track === activeTrack) {
        if (track.seeking) {
            track.seekTo = seekTo;
        } else if (track.audio.paused) {
            track.seekTo = seekTo;
            play(track);
        } else /* track is playing */ {
            track.audio.currentTime = seekTo;
            updatePlayhead(track);
            announcePlayhead(track);
        }
    } else {
        const playNext = () => {
            setActive(track);
            track.seekTo = seekTo;
            play(track);
        };

        reset(activeTrack, playNext);
    }
}

// Completely resets the track to its original unintialized state. The given
// track can be in any possible state (seeking, playing, etc.), all is
// handled by this function.
function reset(track, onComplete = null) {
    const resetCallback = () => {
        // Remove emphasis in the track list
        track.container.classList.remove('active');

        if (track.open) {
            // Reset "playback heads" back to the beginning
            track.audio.currentTime = 0;
            updatePlayhead(track, true);
            announcePlayhead(track);
            track.open = false;
        }

        // Remove any seekTo state
        delete track.seekTo;

        if (onComplete !== null) {
            onComplete();
        }
    };

    // Another track is active, so we either abort its seeking (if applies) or
    // pause it (if necessary) and reset it. Then we start the new track.
    if (track.seeking) {
        track.seeking.cancel();
        resetCallback();
    } else if (track.audio.paused) {
        resetCallback();
    } else {
        // The pause event occurs with a delay, so we defer resetting the track
        // and starting the next one until just after the pause event fires.
        track.onPause = resetCallback;
        track.audio.pause();
    }
}

function seek(track, onComplete = null) {
    const seeking = { onComplete };

    let closestPerformedSeek = 0;

    function tryFinishSeeking() {
        let closestAvailableSeek = 0;
        const { seekable } = track.audio;
        for (let index = 0; index < seekable.length; index++) {
            if (seekable.start(index) <= track.seekTo) {
                if (seekable.end(index) >= track.seekTo) {
                    track.audio.currentTime = track.seekTo;

                    const { onComplete } = track.seeking;

                    delete track.seeking;
                    delete track.seekTo;
                    clearInterval(seekInterval);

                    if (onComplete !== null) {
                        onComplete();
                    }
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
        if (track.seeking && closestAvailableSeek - closestPerformedSeek > 1) {
            track.audio.currentTime = closestAvailableSeek;
            closestPerformedSeek = closestAvailableSeek;
        }
    }

    const seekInterval = setInterval(tryFinishSeeking, 30);

    seeking.cancel = () => {
        clearInterval(seekInterval);
        delete track.seeking;
        dockedPlayer.playbackButton.replaceChildren(playIcon.cloneNode(true));
        track.container.classList.remove('active');
        track.playbackButtonIcon.replaceChildren(playIcon.cloneNode(true));
        listenButtonIcon.replaceChildren(playIcon.cloneNode(true));
        listenButtonLabel.textContent = PLAYER_JS_T.listen;
    };

    // We expose `cancel` and `onComplete` on the seeking object (and `seekTo`
    // on track itself) so that consecutive parallel playback/seek requests
    // may either cancel seeking (by calling `track.seeking.cancel()`) or
    // reconfigure up to which time seeking should occur (by setting
    // `track.seekTo = newSeekPoint`), or reconfigure what should happen
    // after seeking completes (by setting `track.onComplete = callback).
    track.seeking = seeking;
}

function setActive(track) {
    activeTrack = track;

    // Emphasize active track in the track list.
    track.container.classList.add('active');
}

function setSpeed(track) {
    // Our internal speed representation is in percent so we translate to a
    // multiplication factor here
    track.audio.playbackRate = speed / 100;
}

function setVolume(track) {
    if (volume.finegrained) {
        track.audio.muted = false;
        track.audio.volume = volume.level;
    } else {
        track.audio.muted = (volume.level === 0);
    }
}

function toggleMute() {
    if (volume.level > 0) {
        volume.restoreLevel = volume.level;
        volume.level = 0;
    } else if (volume.restoreLevel) {
        volume.level = volume.restoreLevel;
        delete volume.restoreLevel;
    } else {
        volume.level = 1;
    }
    updateVolume();
}

function updatePlayhead(track, reset = false) {
    const { audio } = track;
    const factor = reset ? 0 : audio.currentTime / track.duration;

    dockedPlayer.progress.style.setProperty('width', `${factor * 100}%`);
    dockedPlayer.currentTime.textContent = formatTime(audio.currentTime);
    dockedPlayer.timelineInput.value = audio.currentTime;

    if (track.waveform) {
        track.waveform.svg.querySelector('linearGradient.playback stop:nth-child(1)').setAttribute('offset', factor);
        track.waveform.svg.querySelector('linearGradient.playback stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
        track.waveform.input.value = audio.currentTime;
    }
}

function updateSpeed() {
    dockedPlayer.speedMultiplier.textContent = (speed / 100).toFixed(1);
    setSpeed(activeTrack);
}

function updateVolume(persist = true) {
    // We may only unmute and make the track audible if its playback is
    // currently solicited by us (see other comments on solicitedPlaback).
    if (activeTrack && activeTrack.solicitedPlayback) {
        setVolume(activeTrack);
    }

    // Render volume button

    const RADIUS = 32;
    const degToRad = deg => (deg * Math.PI) / 180;

    // Compute a path's d attribute for a ring segment.
    // In clock terms we start at 12 o'clock and we go clockwise.
    const segmentD = (beginAngle, arcAngle) => {
        let largeArcFlag = arcAngle < 180 ? 0 : 1 ;

        let beginAngleRad = degToRad(beginAngle);
        let beginX = Math.sin(beginAngleRad);
        let beginY = -Math.cos(beginAngleRad);

        let endAngleRad = degToRad(beginAngle + arcAngle);
        let endX = Math.sin(endAngleRad);
        let endY = -Math.cos(endAngleRad);

        const outerRadius = RADIUS;
        let segmentOuterBeginX = RADIUS + beginX * outerRadius;
        let segmentOuterBeginY = RADIUS + beginY * outerRadius;

        let segmentOuterEndX = RADIUS + endX * outerRadius;
        let segmentOuterEndY = RADIUS + endY * outerRadius;

        let innerRadius = RADIUS * 0.8;
        let segmentInnerBeginX = RADIUS + beginX * innerRadius;
        let segmentInnerBeginY = RADIUS + beginY * innerRadius;

        let segmentInnerEndX = RADIUS + endX * innerRadius;
        let segmentInnerEndY = RADIUS + endY * innerRadius;

        return `
            M ${segmentOuterBeginX},${segmentOuterBeginY}
            A ${outerRadius} ${outerRadius} 0 ${largeArcFlag} 1 ${segmentOuterEndX},${segmentOuterEndY}
            L ${segmentInnerEndX},${segmentInnerEndY}
            A ${innerRadius} ${innerRadius} 0 ${largeArcFlag} 0 ${segmentInnerBeginX},${segmentInnerBeginY}
            Z
        `;
    };

    const displayedLevel = volume.finegrained ? volume.level : (volume.level > 0 ? 1 : 0);

    dockedPlayer.volume.knob.classList.toggle('muted', displayedLevel === 0);
    dockedPlayer.volume.knobSvgTitle.textContent = displayedLevel > 0 ? PLAYER_JS_T.mute : PLAYER_JS_T.unmute;

    const beginAngle = -135;
    const arcAngle = displayedLevel * 270;

    const knobAngle = beginAngle + arcAngle;
    dockedPlayer.volume.knob.querySelector('path.knob').setAttribute('transform', `rotate(${knobAngle} 32 32)`);

    const activeD = displayedLevel > 0 ? segmentD(beginAngle, arcAngle) : '';
    dockedPlayer.volume.knob.querySelector('path.active_range').setAttribute('d', activeD);

    const inactiveD = displayedLevel < 1 ? segmentD(beginAngle + arcAngle, 270 - arcAngle) : '';
    dockedPlayer.volume.knob.querySelector('path.inactive_range').setAttribute('d', inactiveD);

    const percent = displayedLevel * 100;
    const percentFormatted = percent % 1 > 0.1 ? (Math.trunc(percent * 10) / 10) : Math.trunc(percent);
    dockedPlayer.volume.sliderInput.setAttribute('aria-valuetext', `${PLAYER_JS_T.volume} ${percentFormatted}%`);
    dockedPlayer.volume.sliderInput.value = displayedLevel;

    // Render volume slider

    // TODO: Pre-store the two querySelector results (also elsewhere)
    dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level stop:nth-child(1)').setAttribute('offset', displayedLevel);
    dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level stop:nth-child(2)').setAttribute('offset', displayedLevel + 0.0001);

    // These are be re-applied from the respective mousemove handler if
    // needed, until then they are removed to avoid potential visual state
    // indication conflicts (as were observed).
    dockedPlayer.volume.slider.classList.remove('decrease', 'increase');

    if (persist) {
        localStorage.setItem('faircampVolume', volume.level.toString());
    }
}

dockedPlayer.container.addEventListener('keydown', event => {
    if (event.target === dockedPlayer.volume.sliderInput) return;

    if (event.key === 'ArrowLeft') {
        event.preventDefault();
        const seekTo = Math.max(0, activeTrack.audio.currentTime - 5);
        requestSeek(activeTrack, seekTo);
    } else if (event.key === 'ArrowRight') {
        event.preventDefault();
        const seekTo = Math.min(activeTrack.duration - 1, activeTrack.audio.currentTime + 5);
        requestSeek(activeTrack, seekTo);
    }
});

dockedPlayer.playbackButton.addEventListener('click', () => {
    requestPlaybackChange(activeTrack);
});

// Not available on a track player
if (dockedPlayer.nextTrackButton) {
    dockedPlayer.nextTrackButton.addEventListener('click', () => {
        if (activeTrack.nextTrack) {
            requestPlaybackChange(activeTrack.nextTrack);
        }
    });
}

// Only available when enabled
if (dockedPlayer.speedButton) {
    dockedPlayer.speedButton.addEventListener('auxclick', event => {
        event.preventDefault();
        speed = 100;
        updateSpeed();
    });

    dockedPlayer.speedButton.addEventListener('click', () => {
        if (speed < 100) {
            speed = 100;
        } else if (speed < 120) {
            speed = 120;
        } else if (speed < 140) {
            speed = 140;
        } else if (speed < 160) {
            speed = 160;
        } else if (speed < 180) {
            speed = 180;
        } else if (speed < 200) {
            speed = 200;
        } else {
            speed = 100;
        }

        updateSpeed();
    });

    // Prevent context menu from opening when using right-click to reset
    // playback speed.
    dockedPlayer.speedButton.addEventListener('contextmenu', event => event.preventDefault());

    dockedPlayer.speedButton.addEventListener('keydown', event => {
        if (event.key === 'ArrowDown' && speed > 30) {
            speed -= 10;
        } else if (event.key === 'ArrowUp' && speed < 300) {
            speed += 10;
        } else {
            return;
        }

        updateSpeed();

        event.preventDefault();
        event.stopPropagation();
    });

    dockedPlayer.speedButton.addEventListener('wheel', event => {
        if (event.deltaY < 0 && speed < 300) {
            speed += 10;
        } else if (event.deltaY > 0 && speed > 30) {
            speed -= 10;
        }

        updateSpeed();

        event.preventDefault();
    });
}

dockedPlayer.timeline.addEventListener('click', () => {
    const factor = (event.clientX - dockedPlayer.timeline.getBoundingClientRect().x) / dockedPlayer.timeline.getBoundingClientRect().width;
    const seekTo = factor * dockedPlayer.timelineInput.max;
    requestSeek(activeTrack, seekTo);
    dockedPlayer.timeline.classList.add('focus_from_click');
    dockedPlayer.timelineInput.focus();
});

dockedPlayer.timelineInput.addEventListener('blur', () => {
    dockedPlayer.timeline.classList.remove('focus', 'focus_from_click');
});

dockedPlayer.timelineInput.addEventListener('focus', () => {
    dockedPlayer.timeline.classList.add('focus');
});

dockedPlayer.timelineInput.addEventListener('keydown', event => {
    if (event.key === ' ' || event.key === 'Enter') {
        event.preventDefault();
        requestPlaybackChange(activeTrack);
    }
});

dockedPlayer.volume.container.addEventListener('auxclick', event => {
    event.preventDefault();
    volume.level = volume.level < 1 ? 1 : 0;
    updateVolume();
});

// Prevent context menu from opening when using right-click to reset volume.
dockedPlayer.volume.container.addEventListener('contextmenu', event => event.preventDefault());

dockedPlayer.volume.container.addEventListener('mouseenter', () => {
    // On Chromium we observed that in mobile device simulation, touch events
    // on entirely unrelated elements (e.g. the global "Listen" button) could
    // trigger mouseenter/mouseleave events on the volume container.
    // Therefore we make sure not to process these events when there is no
    // hover capability.
    if (hoverAvailable()) {
        dockedPlayer.volume.container.classList.add('hover');
    }
});

dockedPlayer.volume.container.addEventListener('mouseleave', () => {
    // See note for mouseenter
    if (hoverAvailable()) {
        dockedPlayer.volume.container.classList.remove('hover');
        dockedPlayer.volume.slider.classList.remove('decrease', 'increase');
    }
});

dockedPlayer.volume.container.addEventListener('wheel', event => {
    event.preventDefault();

    if (volume.finegrained) {
        volume.level += event.deltaY * -0.0001;

        if (volume.level > 1) {
            volume.level = 1;
        } else if (volume.level < 0) {
            volume.level = 0;
        }
    } else {
        if (event.deltaY < 0) {
            volume.level = 1;
        } else if (event.deltaY > 0) {
            volume.level = 0;
        }
    }

    updateVolume();
});

dockedPlayer.volume.knob.addEventListener('click', () => {
    if (hoverAvailable() || !volume.finegrained) {
        toggleMute();
    } else {
        // On mobiles/tablets with finegrained volume but without hover
        // capability, hover (for revealing the volume slider) is emulated by
        // clicking the volume knob.
        dockedPlayer.volume.container.classList.toggle('hover');
    }
});

dockedPlayer.volume.knob.addEventListener('keydown', event => {
    if (event.key === 'ArrowDown') {
        volume.level -= 0.02;
    } else if (event.key === 'ArrowUp') {
        volume.level += 0.02;
    } else {
        return;
    }

    if (volume.level > 1) {
        volume.level = 1;
    } else if (volume.level < 0) {
        volume.level = 0;
    }

    updateVolume();

    event.preventDefault();
    event.stopPropagation();
});

dockedPlayer.volume.slider.addEventListener('mousedown', event => {
    if (event.button === 0 /* === primary button */) {
        const clickAndDragHandler = event => {
            const svgRect = dockedPlayer.volume.sliderSvg.getBoundingClientRect()
            const clampedMouseX = Math.min(Math.max(0, event.clientX - svgRect.x), svgRect.width);
            volume.level = clampedMouseX / svgRect.width;
            updateVolume();
        };

        clickAndDragHandler(event);

        window.addEventListener('mousemove', clickAndDragHandler);
        dockedPlayer.volume.slider.classList.add('dragging');

        window.addEventListener('mouseup', () => {
            window.removeEventListener('mousemove', clickAndDragHandler);
            dockedPlayer.volume.slider.classList.remove('dragging');
        }, { once: true });
    }
});

dockedPlayer.volume.slider.addEventListener('mousemove', event => {
    const svgRect = dockedPlayer.volume.sliderSvg.getBoundingClientRect()
    const clampedMouseX = Math.min(Math.max(0, event.clientX - svgRect.x), svgRect.width);
    const mouseLevel = clampedMouseX / svgRect.width;

    if (mouseLevel > volume.level) {
        dockedPlayer.volume.slider.classList.remove('decrease');
        dockedPlayer.volume.slider.classList.add('increase');
        dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level_increase stop:nth-child(1)').setAttribute('offset', mouseLevel);
        dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level_increase stop:nth-child(2)').setAttribute('offset', mouseLevel + 0.0001);
    } else {
        dockedPlayer.volume.slider.classList.remove('increase');
        dockedPlayer.volume.slider.classList.add('decrease');
        dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level_decrease stop:nth-child(1)').setAttribute('offset', mouseLevel);
        dockedPlayer.volume.slider.querySelector('linearGradient#gradient_level_decrease stop:nth-child(2)').setAttribute('offset', mouseLevel + 0.0001);
    }

});

const handleTouch = event => {
    for (const touch of event.changedTouches) {
        const svgRect = dockedPlayer.volume.sliderSvg.getBoundingClientRect()
        const clampedTouchX = Math.min(Math.max(0, touch.clientX - svgRect.x), svgRect.width);
        volume.level = clampedTouchX / svgRect.width;
        updateVolume();
    }
};

dockedPlayer.volume.slider.addEventListener('touchcancel', handleTouch);
dockedPlayer.volume.slider.addEventListener('touchend', handleTouch);
dockedPlayer.volume.slider.addEventListener('touchmove', handleTouch);
dockedPlayer.volume.slider.addEventListener('touchstart', handleTouch);

dockedPlayer.volume.sliderInput.addEventListener('blur', () => {
    dockedPlayer.volume.slider.classList.remove('focus');
});

dockedPlayer.volume.sliderInput.addEventListener('focus', () => {
    dockedPlayer.volume.slider.classList.add('focus');
});

dockedPlayer.volume.sliderInput.addEventListener('input', () => {
    volume.level = parseFloat(dockedPlayer.volume.sliderInput.valueAsNumber);
    updateVolume();
});

// This was observed to jump between 0 and 1 without a single step in between,
// hence we disable the default behavior and handle it ourselves
dockedPlayer.volume.sliderInput.addEventListener('keydown', event => {
    if (event.key === 'ArrowLeft' || event.key === 'ArrowDown') {
        volume.level -= 0.02;
    } else if (event.key === 'ArrowRight' || event.key === 'ArrowUp') {
        volume.level += 0.02;
    } else if (event.key === 'Enter' || event.key === ' ') {
        toggleMute();
    } else {
        return;
    }

    if (volume.level > 1) {
        volume.level = 1;
    } else if (volume.level < 0) {
        volume.level = 0;
    }

    updateVolume();

    event.preventDefault();
});

// This was observed to "scroll" between 0 and 1 without a single step in between,
// hence we disable the default behavior and let the event bubble up to our own handler
dockedPlayer.volume.sliderInput.addEventListener('wheel', event => event.preventDefault());

listenButton.addEventListener('click', () => {
    requestPlaybackChange(activeTrack);
});

if (navigator.mediaSession) {
    navigator.mediaSession.setActionHandler('play', () => {
        requestPlaybackChange(activeTrack);
    });

    navigator.mediaSession.setActionHandler('pause', () => {
        requestPlaybackChange(activeTrack);
    });
}

const resizeObserver = new ResizeObserver(entries => {
    const minWidth = entries.reduce(
        (minWidth, entry) => Math.min(entry.contentRect.width, minWidth),
        Infinity
    );

    waveforms(minWidth);
});

let previousTrack = null;
let trackIndex = 0;
for (const container of document.querySelectorAll('.track')) {
    const artists = container.querySelector('.artists');
    const audio = container.querySelector('audio');
    const number = container.querySelector('.number');
    const playbackButton = container.querySelector('.track_playback');
    const playbackButtonIcon = container.querySelector('.track_playback .icon');
    const title = container.querySelector('.title');

    const duration = parseFloat(container.dataset.duration);

    const track = {
        artists,
        audio,
        container,
        duration,
        number,
        playbackButton,
        playbackButtonIcon,
        title
    };

    // We only unmute tracks right before they play, muting them again at any
    // pause event. We do this because a bug in browsers on apple systems can
    // trigger sporadic, unsolicited playback of tracks in certain conditions
    // (see comment elsewhere on track.solicitedPlayback), and although we
    // cancel this unsolicited playback right away, it would be sometimes
    // audible for a brief moment (if we didn't keep tracks muted).
    audio.muted = true;

    // Playback buttons start off with tabindex="-1" because if the visitor
    // has JavaScript disabled the element should not be interacted with at
    // all. When JavaScript is available we revert to making the button
    // reachable by keyboard.
    playbackButton.tabIndex = 0;

    if (previousTrack !== null) {
        previousTrack.nextTrack = track;
    }

    previousTrack = track;

    audio.addEventListener('ended', event => {
        if (track.nextTrack) {
            requestPlaybackChange(track.nextTrack);
        } else {
            reset(track);
            // Hide docked player
            document.body.classList.remove('player_active');
            dockedPlayer.status.setAttribute('aria-label', PLAYER_JS_T.playerClosed);
            setActive(tracks[0]);
        }
    });

    audio.addEventListener('pause', event => {
        if (!track.solicitedPlayback) { return; }

        delete track.solicitedPlayback;
        track.audio.muted = true;

        clearInterval(globalUpdatePlayHeadInterval);

        container.classList.remove('playing');
        dockedPlayer.playbackButton.replaceChildren(playIcon.cloneNode(true));
        listenButtonIcon.replaceChildren(playIcon.cloneNode(true));
        listenButtonLabel.textContent = PLAYER_JS_T.listen;
        track.playbackButtonIcon.replaceChildren(playIcon.cloneNode(true));

        if (track.onPause) {
            track.onPause();
            delete track.onPause;
        } else {
            updatePlayhead(track);
            announcePlayhead(track);
        }
    });

    audio.addEventListener('play', event => {
        if (!track.solicitedPlayback) {
            // Unsolicited playback triggered by Apple/Safari (see comment
            // elsewhere regarding track.solicitedPlayback), we cancel it
            // immediately.
            audio.pause();
            return;
        }

        container.classList.add('playing');
        dockedPlayer.playbackButton.replaceChildren(pauseIcon.cloneNode(true));
        listenButtonIcon.replaceChildren(pauseIcon.cloneNode(true));
        listenButtonLabel.textContent = PLAYER_JS_T.pause;
        track.playbackButtonIcon.replaceChildren(pauseIcon.cloneNode(true));

        globalUpdatePlayHeadInterval = setInterval(() => updatePlayhead(track), 1000 / 24);
        updatePlayhead(track);
        announcePlayhead(track);
    });

    audio.addEventListener('playing', event => {
        if (!track.solicitedPlayback) { return; }

        dockedPlayer.playbackButton.replaceChildren(pauseIcon.cloneNode(true));
        listenButtonIcon.replaceChildren(pauseIcon.cloneNode(true));
        listenButtonLabel.textContent = PLAYER_JS_T.pause;
        track.playbackButtonIcon.replaceChildren(pauseIcon.cloneNode(true));
    });

    audio.addEventListener('waiting', event => {
        if (!track.solicitedPlayback) { return; }

        // TODO: Eventually we could augment various screenreader labels here to
        //       indicate the loading state too
        dockedPlayer.playbackButton.replaceChildren(loadingIcon.cloneNode(true));
        listenButtonIcon.replaceChildren(loadingIcon.cloneNode(true));
        listenButtonLabel.textContent = PLAYER_JS_T.pause;
        track.playbackButtonIcon.replaceChildren(loadingIcon.cloneNode(true));
    });

    track.playbackButton.addEventListener('click', event => {
        event.preventDefault();
        requestPlaybackChange(track);
    });

    container.addEventListener('keydown', event => {
        if (event.key === 'ArrowLeft') {
            event.preventDefault();
            const seekTo = Math.max(0, track.audio.currentTime - 5);
            requestSeek(track, seekTo);
        } else if (event.key === 'ArrowRight') {
            event.preventDefault();
            const seekTo = Math.min(track.duration - 1, track.audio.currentTime + 5);
            requestSeek(track, seekTo);
        }
    });

    const waveformContainer = container.querySelector('.waveform');
    if (waveformContainer) {
        const input = waveformContainer.querySelector('.waveform input');
        const svg = waveformContainer.querySelector('.waveform svg');

        const peaks = decode(svg.dataset.peaks).map(peak => peak / 63);

        track.waveform = {
            container: waveformContainer,
            input,
            peaks,
            svg
        };

        track.waveform.container.addEventListener('click', event => {
            const factor = (event.clientX - track.waveform.input.getBoundingClientRect().x) / track.waveform.input.getBoundingClientRect().width;
            const seekTo = factor * track.waveform.input.max
            requestSeek(track, seekTo);
            track.waveform.input.classList.add('focus_from_click');
            track.waveform.input.focus();
        });

        track.waveform.container.addEventListener('mouseenter', event => {
            track.waveform.container.classList.add('seek');
        });

        track.waveform.container.addEventListener('mousemove', event => {
            const factor = (event.clientX - track.waveform.container.getBoundingClientRect().x) / track.waveform.container.getBoundingClientRect().width;
            // TODO: Pre-store the two querySelector results
            track.waveform.svg.querySelector('linearGradient.seek stop:nth-child(1)').setAttribute('offset', factor);
            track.waveform.svg.querySelector('linearGradient.seek stop:nth-child(2)').setAttribute('offset', factor + 0.0001);
        });

        track.waveform.container.addEventListener('mouseout', event => {
            track.waveform.container.classList.remove('seek');
        });

        track.waveform.input.addEventListener('blur', () => {
            track.waveform.input.classList.remove('focus_from_click');
        });

        track.waveform.input.addEventListener('focus', () => {
            announcePlayhead(track);
        });

        track.waveform.input.addEventListener('keydown', event => {
            if (event.key === ' ' || event.key === 'Enter') {
                event.preventDefault();
                requestPlaybackChange(track);
            }
        });

        // Initialize playback/seek gradients
        const SVG_XMLNS = 'http://www.w3.org/2000/svg';

        svg.setAttribute('xmlns', SVG_XMLNS);
        svg.setAttribute('height', `${TRACK_HEIGHT_EM}em`);

        const defs = document.createElementNS(SVG_XMLNS, 'defs');

        const playbackGradient = document.createElementNS(SVG_XMLNS, 'linearGradient');
        playbackGradient.classList.add('playback');
        playbackGradient.id = `gradient_playback_${trackIndex}`;
        const playbackGradientStop1 = document.createElementNS(SVG_XMLNS, 'stop');
        playbackGradientStop1.setAttribute('offset', '0');
        playbackGradientStop1.setAttribute('stop-color', 'var(--fg-1)');
        const playbackGradientStop2 = document.createElementNS(SVG_XMLNS, 'stop');
        playbackGradientStop2.setAttribute('offset', '0.000001');
        playbackGradientStop2.setAttribute('stop-color', 'hsla(0, 0%, 0%, 0)');
        playbackGradient.append(playbackGradientStop1, playbackGradientStop2);

        const seekGradient = document.createElementNS(SVG_XMLNS, 'linearGradient');
        seekGradient.classList.add('seek');
        seekGradient.id = `gradient_seek_${trackIndex}`;
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

        svg.querySelector('path.playback').setAttribute('stroke', `url(#gradient_playback_${trackIndex})`);
        svg.querySelector('path.seek').setAttribute('stroke', `url(#gradient_seek_${trackIndex})`);

        // Trigger waveform recomputation on resize (this also triggers the
        // first draw after the initial page load).
        const waveformParent = track.waveform.container.parentElement;
        resizeObserver.observe(waveformParent);
    }

    trackIndex++;
    tracks.push(track);
}

window.addEventListener('hashchange', event => {
    const params = parseHashParams();

    if (params) {
        requestSeek(params.track, params.time ?? 0);
        // TODO: This can be observed as a brief flicker in the address bar
        // (when you look for it). Possibly look for a more elegant way in
        // the future.
        history.replaceState(null, '', ' ');
        event.preventDefault();
    }
});

window.addEventListener('keydown', event => {
    if (event.key === 'm') {
        toggleMute();
        event.preventDefault();
    }
});

// Set active track (and optionally set seekTo and/or open player)
const params = parseHashParams();
if (params) {
    setActive(params.track);

    if (params.time !== undefined) {
        params.track.seekTo = params.time;
    }

    open(params.track);

    // Announce to screenreaders that the docked player is present
    dockedPlayer.status.setAttribute('aria-label', PLAYER_JS_T.playerOpenWithXxx(params.track.title.textContent));
} else {
    setActive(tracks[0]);
}

const waveformRenderState = { widthRem: 0 };

function waveforms(minWidth) {
    const baseFontSizePx = parseFloat(
        window.getComputedStyle(document.documentElement)
              .getPropertyValue('font-size')
              .replace('px', '')
    );

    let maxWaveformWidthRem = (minWidth - WAVEFORM_WIDTH_PADDING_REM) / baseFontSizePx;
    let relativeWaveforms = maxWaveformWidthRem > BREAKPOINT_REDUCED_WAVEFORM_REM && !document.querySelector('[data-disable-relative-waveforms]');

    if (waveformRenderState.widthRem >= maxWaveformWidthRem - WAVEFORM_WIDTH_TOLERANCE_REM &&
        waveformRenderState.widthRem <= maxWaveformWidthRem + WAVEFORM_WIDTH_TOLERANCE_REM) return;

    const longestTrackDuration = parseFloat(document.querySelector('[data-longest-duration]').dataset.longestDuration);

    for (const track of tracks) {
        const peaks = track.waveform.peaks;
        const trackDuration = parseFloat(track.waveform.input.max);

        let waveformWidthRem = maxWaveformWidthRem;

        if (relativeWaveforms) {
            waveformWidthRem *= (trackDuration / longestTrackDuration);
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

        const svg = track.waveform.svg;

        svg.setAttribute('viewBox', `0 0 ${waveformWidthRem} 1.5`);
        svg.setAttribute('width', `${waveformWidthRem}em`);
        svg.querySelector('path.base').setAttribute('d', d);
        svg.querySelector('path.playback').setAttribute('d', d);
        svg.querySelector('path.seek').setAttribute('d', d);
    }

    waveformRenderState.widthRem = maxWaveformWidthRem;
}
