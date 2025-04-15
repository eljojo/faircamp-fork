const loadingIcon = document.querySelector('#loading_icon').content;
const pauseIcon = document.querySelector('#pause_icon').content;
const playIcon = document.querySelector('#play_icon').content;

// There is always an active track (so this is guaranteed to be available
// after the respective intialization routines have completed). "active"
// refers to various states, but it generally indicates that the track is
// selected for being played back, open in the player, in the process of
// loading/seeking, or already being played back. Conversely, any not active
// track is guaranteed to be cleaned up in terms of its state, i.e. not being
// played back, not being displayed (in a place where it needs track-specific
// clean-up), not running asynchronous routines (that don't clean themselves
// up invisibly when they finish), etc. Differentiation of the exact state of
// an active track happens through properties that are set on it, see the
// respective parts in the code.
let activeTrack;

const tracks = [];

const playerContainer = document.querySelector('.player');
const player = {
    container: playerContainer,
    currentTime: playerContainer.querySelector('.time .current'),
    nextTrackButton: playerContainer.querySelector('button.next_track'),
    number: playerContainer.querySelector('.number'),
    playbackButton: playerContainer.querySelector('button.playback'),
    previousTrackButton: playerContainer.querySelector('button.previous_track'),
    progress: playerContainer.querySelector('.progress'),
    speedButton: playerContainer.querySelector('button.speed'),
    speedMultiplier: playerContainer.querySelector('button.speed .multiplier'),
    timeline: playerContainer.querySelector('.timeline'),
    timelineInput: playerContainer.querySelector('.timeline input'),
    titleWrapper: playerContainer.querySelector('.title_wrapper'),
    totalTime: playerContainer.querySelector('.time .total'),
    volume: {
        container: playerContainer.querySelector('.volume'),
        knob: playerContainer.querySelector('.volume button'),
        knobSvgTitle: playerContainer.querySelector('.volume button svg title'),
        slider: playerContainer.querySelector('.volume .slider'),
        sliderInput: playerContainer.querySelector('.volume .slider input'),
        sliderSvg: playerContainer.querySelector('.volume .slider svg')
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
    player.volume.container.classList.add('finegrained');
    volume.finegrained = true;
    updateVolume(false);
    volumeProbe.removeEventListener('volumechange', volumeProbeHandler);
    volumeProbe = null;
};
volumeProbe.addEventListener('volumechange', volumeProbeHandler);
volumeProbe.volume = 0.123;

const persistedVolume = localStorage.getItem('faircampEmbedVolume');
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
    const valueText = `${EMBEDS_JS_T.playbackPosition} ${formatTimeWrittenOut(track.input.value)}`;
    player.timelineInput.setAttribute('aria-valuetext', valueText);
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
        return EMBEDS_JS_T.xxxSeconds(Math.floor(seconds));
    } else {
        const secondsWrittenOut = EMBEDS_JS_T.xxxSeconds(Math.floor(Math.floor(seconds % 60)));
        if (seconds < 3600) {
            return `${EMBEDS_JS_T.xxxMinutes(Math.floor(seconds / 60))} ${secondsWrittenOut}`;
        } else {
            return `${EMBEDS_JS_T.xxxHours(Math.floor(seconds / 3600))} ${EMBEDS_JS_T.xxxMinutes(Math.floor((seconds % 3600) / 60))} ${secondsWrittenOut}`;
        }
    }
}

// Open the docked player and update its various subelements to display the
// given track. If track.seekTo is set a seek is indicated by advancing both
// the track's own progress indicator and the docked player progress bar to
// the seek point (that seek however isn't performed yet, that's only done
// when playback is initiated).
function open(track) {
    player.currentTime.textContent = formatTime(track.seekTo ?? track.audio.currentTime);
    player.totalTime.textContent = formatTime(track.duration);
    player.timelineInput.max = track.container.dataset.duration;

    if (track.artists) {
        player.titleWrapper.replaceChildren(track.title.cloneNode(true), track.artists.cloneNode(true));
    } else {
        player.titleWrapper.replaceChildren(track.title.cloneNode(true));
    }

    // Not available on a track player
    if (player.number) {
        player.nextTrackButton.toggleAttribute('disabled', !track.nextTrack);
        player.number.textContent = track.number.textContent;
        player.previousTrackButton.toggleAttribute('disabled', !track.previousTrack);
    }

    if (track.seekTo !== undefined && track.seekTo > 0) {
        const factor = track.seekTo / track.duration;

        player.progress.style.setProperty('width', `${factor * 100}%`);
        player.currentTime.textContent = formatTime(track.seekTo);
        player.timelineInput.value = track.seekTo;
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
    player.playbackButton.replaceChildren(loadingIcon.cloneNode(true));

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
        player.playbackButton.replaceChildren(playIcon.cloneNode(true));
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

    player.progress.style.setProperty('width', `${factor * 100}%`);
    player.currentTime.textContent = formatTime(audio.currentTime);
    player.timelineInput.value = audio.currentTime;
}

function updateSpeed() {
    player.speedMultiplier.textContent = (speed / 100).toFixed(1);
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

    player.volume.knob.classList.toggle('muted', displayedLevel === 0);
    player.volume.knobSvgTitle.textContent = displayedLevel > 0 ? EMBEDS_JS_T.mute : EMBEDS_JS_T.unmute;

    const beginAngle = -135;
    const arcAngle = displayedLevel * 270;

    const knobAngle = beginAngle + arcAngle;
    player.volume.knob.querySelector('path.knob').setAttribute('transform', `rotate(${knobAngle} 32 32)`);

    const activeD = displayedLevel > 0 ? segmentD(beginAngle, arcAngle) : '';
    player.volume.knob.querySelector('path.active_range').setAttribute('d', activeD);

    const inactiveD = displayedLevel < 1 ? segmentD(beginAngle + arcAngle, 270 - arcAngle) : '';
    player.volume.knob.querySelector('path.inactive_range').setAttribute('d', inactiveD);

    const percent = displayedLevel * 100;
    const percentFormatted = percent % 1 > 0.1 ? (Math.trunc(percent * 10) / 10) : Math.trunc(percent);
    player.volume.sliderInput.setAttribute('aria-valuetext', `${EMBEDS_JS_T.volume} ${percentFormatted}%`);
    player.volume.sliderInput.value = displayedLevel;

    // Render volume slider

    // TODO: Pre-store the two querySelector results (also elsewhere)
    player.volume.slider.querySelector('linearGradient#gradient_level stop:nth-child(1)').setAttribute('offset', displayedLevel);
    player.volume.slider.querySelector('linearGradient#gradient_level stop:nth-child(2)').setAttribute('offset', displayedLevel + 0.0001);

    // These are be re-applied from the respective mousemove handler if
    // needed, until then they are removed to avoid potential visual state
    // indication conflicts (as were observed).
    player.volume.slider.classList.remove('decrease', 'increase');

    if (persist) {
        localStorage.setItem('faircampEmbedVolume', volume.level.toString());
    }
}

player.container.addEventListener('keydown', event => {
    if (event.target === player.volume.sliderInput) return;

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

player.playbackButton.addEventListener('click', () => {
    requestPlaybackChange(activeTrack);
});

// Not available on a track player
if (player.nextTrackButton) {
    player.nextTrackButton.addEventListener('click', () => {
        if (activeTrack?.nextTrack) {
            requestPlaybackChange(activeTrack.nextTrack);
        }
    });
}

// Not available on a track player
if (player.previousTrackButton) {
    player.previousTrackButton.addEventListener('click', () => {
        if (activeTrack?.previousTrack) {
            requestPlaybackChange(activeTrack.previousTrack);
        }
    });
}

// Only available when enabled
if (player.speedButton) {
    player.speedButton.addEventListener('auxclick', event => {
        event.preventDefault();
        speed = 100;
        updateSpeed();
    });

    player.speedButton.addEventListener('click', () => {
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
    player.speedButton.addEventListener('contextmenu', event => event.preventDefault());

    player.speedButton.addEventListener('keydown', event => {
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

    player.speedButton.addEventListener('wheel', () => {
        if (event.deltaY < 0 && speed < 300) {
            speed += 10;
        } else if (event.deltaY > 0 && speed > 30) {
            speed -= 10;
        }

        updateSpeed();
    });
}

player.timeline.addEventListener('click', () => {
    const factor = (event.clientX - player.timeline.getBoundingClientRect().x) / player.timeline.getBoundingClientRect().width;
    const seekTo = factor * player.timelineInput.max;
    requestSeek(activeTrack, seekTo);
    player.timeline.classList.add('focus_from_click');
    player.timelineInput.focus();
});

player.timelineInput.addEventListener('blur', () => {
    player.timeline.classList.remove('focus', 'focus_from_click');
});

player.timelineInput.addEventListener('focus', () => {
    player.timeline.classList.add('focus');
});

player.timelineInput.addEventListener('keydown', event => {
    if (event.key === ' ' || event.key === 'Enter') {
        event.preventDefault();
        requestPlaybackChange(activeTrack);
    }
});

player.volume.container.addEventListener('auxclick', event => {
    event.preventDefault();
    volume.level = volume.level < 1 ? 1 : 0;
    updateVolume();
});

// Prevent context menu from opening when using right-click to reset volume.
player.volume.container.addEventListener('contextmenu', event => event.preventDefault());

player.volume.container.addEventListener('mouseenter', () => {
    player.volume.container.classList.add('hover');
});

player.volume.container.addEventListener('mouseleave', () => {
    player.volume.container.classList.remove('hover');
    player.volume.slider.classList.remove('decrease', 'increase');
});

player.volume.container.addEventListener('wheel', event => {
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

player.volume.knob.addEventListener('click', () => toggleMute());

player.volume.knob.addEventListener('keydown', event => {
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

player.volume.slider.addEventListener('mousedown', event => {
    if (event.button === 0 /* === primary button */) {
        const clickAndDragHandler = event => {
            const svgRect = player.volume.sliderSvg.getBoundingClientRect()
            const clampedMouseX = Math.min(Math.max(0, event.clientX - svgRect.x), svgRect.width);
            volume.level = clampedMouseX / svgRect.width;
            updateVolume();
        };

        clickAndDragHandler(event);

        window.addEventListener('mousemove', clickAndDragHandler);
        player.volume.slider.classList.add('dragging');

        window.addEventListener('mouseup', () => {
            window.removeEventListener('mousemove', clickAndDragHandler);
            player.volume.slider.classList.remove('dragging');
        }, { once: true });
    }
});

player.volume.slider.addEventListener('mousemove', event => {
    const svgRect = player.volume.sliderSvg.getBoundingClientRect()
    const clampedMouseX = Math.min(Math.max(0, event.clientX - svgRect.x), svgRect.width);
    const mouseLevel = clampedMouseX / svgRect.width;

    if (mouseLevel > volume.level) {
        player.volume.slider.classList.remove('decrease');
        player.volume.slider.classList.add('increase');
        player.volume.slider.querySelector('linearGradient#gradient_level_increase stop:nth-child(1)').setAttribute('offset', mouseLevel);
        player.volume.slider.querySelector('linearGradient#gradient_level_increase stop:nth-child(2)').setAttribute('offset', mouseLevel + 0.0001);
    } else {
        player.volume.slider.classList.remove('increase');
        player.volume.slider.classList.add('decrease');
        player.volume.slider.querySelector('linearGradient#gradient_level_decrease stop:nth-child(1)').setAttribute('offset', mouseLevel);
        player.volume.slider.querySelector('linearGradient#gradient_level_decrease stop:nth-child(2)').setAttribute('offset', mouseLevel + 0.0001);
    }

});

player.volume.sliderInput.addEventListener('blur', () => {
    player.volume.slider.classList.remove('focus');
});

player.volume.sliderInput.addEventListener('focus', () => {
    player.volume.slider.classList.add('focus');
});

player.volume.sliderInput.addEventListener('input', () => {
    volume.level = parseFloat(player.volume.sliderInput.valueAsNumber);
    updateVolume();
});

// This was observed to jump between 0 and 1 without a single step in between,
// hence we disable the default behavior and handle it ourselves
player.volume.sliderInput.addEventListener('keydown', event => {
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
player.volume.sliderInput.addEventListener('wheel', event => event.preventDefault());

if (navigator.mediaSession) {
    navigator.mediaSession.setActionHandler('play', () => {
        requestPlaybackChange(activeTrack);
    });

    navigator.mediaSession.setActionHandler('pause', () => {
        requestPlaybackChange(activeTrack);
    });
}

let previousTrack = null;
let trackIndex = 0;
for (const container of document.querySelectorAll('.track')) {
    const artists = container.querySelector('.artists');
    const audio = container.querySelector('audio');
    const input = container.querySelector('input');
    const number = container.querySelector('.number');
    const title = container.querySelector('.title');

    const duration = parseFloat(container.dataset.duration);

    const track = {
        artists,
        audio,
        container,
        duration,
        input,
        number,
        title
    };

    // We only unmute tracks right before they play, muting them again at any
    // pause event. We do this because a bug in browsers on apple systems can
    // trigger sporadic, unsolicited playback of tracks in certain conditions
    // (see comment elsewhere on track.solicitedPlayback), and although we
    // cancel this unsolicited playback right away, it would be sometimes
    // audible for a brief moment (if we didn't keep tracks muted).
    audio.muted = true;

    if (previousTrack !== null) {
        previousTrack.nextTrack = track;
        track.previousTrack = previousTrack;
    }

    previousTrack = track;

    audio.addEventListener('ended', event => {
        audio.currentTime = 0;
        container.classList.remove('active', 'playing');

        if (track.nextTrack) {
            requestPlaybackChange(track.nextTrack);
        } else {
            reset(track);
            setActive(tracks[0]);
            open(tracks[0]);
        }
    });

    audio.addEventListener('pause', event => {
        if (!track.solicitedPlayback) { return; }

        delete track.solicitedPlayback;
        track.audio.muted = true;

        clearInterval(globalUpdatePlayHeadInterval);

        container.classList.remove('playing');
        player.playbackButton.replaceChildren(playIcon.cloneNode(true));

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
        player.playbackButton.replaceChildren(pauseIcon.cloneNode(true));

        globalUpdatePlayHeadInterval = setInterval(() => updatePlayhead(track), 1000 / 24);
        updatePlayhead(track);
        announcePlayhead(track);
    });

    audio.addEventListener('playing', event => {
        if (!track.solicitedPlayback) { return; }

        player.playbackButton.replaceChildren(pauseIcon.cloneNode(true));
    });

    audio.addEventListener('waiting', event => {
        if (!track.solicitedPlayback) { return; }

        // TODO: Eventually we could augment various screenreader labels here to
        //       indicate the loading state too
        player.playbackButton.replaceChildren(loadingIcon.cloneNode(true));
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

    trackIndex++;
    tracks.push(track);
}

window.addEventListener('keydown', event => {
    if (event.key === 'm') {
        toggleMute();
        event.preventDefault();
    }
});

// Set active track (and optionally set seekTo)
const params = parseHashParams();
if (params) {
    setActive(params.track);

    if (params.time !== undefined) {
        params.track.seekTo = params.time;
    }
} else {
    setActive(tracks[0]);
}

open(activeTrack);
