const loadingIcon = document.querySelector('#loading_icon');
const pauseIcon = document.querySelector('#pause_icon');
const playIcon = document.querySelector('#play_icon');

let activeTrack = null;
let firstTrack = null;

const container = document.querySelector('.player');
const player = {
    container,
    nextTrackButton: container.querySelector('button.next_track'),
    number: container.querySelector('.number'),
    playbackButton: container.querySelector('button.playback'),
    previousTrackButton: container.querySelector('button.previous_track'),
    progress: container.querySelector('.progress'),
    time: container.querySelector('.time'),
    timeline: container.querySelector('.timeline'),
    timelineInput: container.querySelector('.timeline input'),
    titleWrapper: container.querySelector('.title_wrapper'),
    volumeButton: container.querySelector('.volume button'),
    volumeInput: container.querySelector('.volume input'),
    volumeSvgTitle: container.querySelector('.volume svg title')
};

let globalUpdatePlayHeadInterval;

const volume = {
    container: document.querySelector('.volume'),
    level: 1
};

const persistedVolume = localStorage.getItem('faircampEmbedVolume');
if (persistedVolume !== null) {
    const level = parseFloat(persistedVolume);
    if (level >= 0 && level <= 1) {
        volume.level = level;
    }
}
updateVolume();

// While the underlying data model of the playhead (technically the invisible
// range input and visible svg representation) change granularly, we only
// trigger screenreader announcements when it makes sense - e.g. when
// focusing the range input, when seeking, when playback ends etc.
function announcePlayhead(track) {
    const valueText = `${T.playbackPosition} ${formatTime(track.input.value)}`;
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

function mount(track) {
    activeTrack = track;

    player.container.classList.add('active');
    player.time.textContent = `0:00 / ${formatTime(track.duration)}`;
    player.timelineInput.max = track.container.dataset.duration;

    if (track.artists) {
        player.titleWrapper.replaceChildren(track.title.cloneNode(true), track.artists.cloneNode(true));
    } else {
        player.titleWrapper.replaceChildren(track.title.cloneNode(true));
    }

    if (player.number) {
        player.number.textContent = track.number.textContent;
    }
}

async function mountAndPlay(track, seekTo) {
    activeTrack = track;

    player.container.classList.add('active');
    player.time.textContent = `0:00 / ${formatTime(track.duration)}`;
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

    updateVolume();

    // The pause and loading icon are visually indistinguishable (until the
    // actual loading animation kicks in after 500ms), hence we right away
    // transistion to the loading icon to make the interface feel snappy,
    // even if we potentially replace it with the pause icon right after that
    // if there doesn't end up to be any loading required.
    track.container.classList.add('active');
    player.playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));

    if (track.audio.preload !== 'auto') {
        track.audio.preload = 'auto';
        track.audio.load();
    }

    const play = () => {
        track.audio.volume = volume.level;
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
            player.playbackButton.replaceChildren(playIcon.content.cloneNode(true));
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

function updatePlayhead(track, reset = false) {
    const { audio } = track;
    const factor = reset ? 0 : audio.currentTime / track.duration;

    player.progress.style.setProperty('width', `${factor * 100}%`);
    player.time.textContent = `${formatTime(audio.currentTime)} / ${formatTime(track.duration)}`;
}

function updateVolume(restoreLevel = null) {
    if (activeTrack) {
        activeTrack.audio.volume = volume.level;
    }

    localStorage.setItem('faircampEmbedVolume', volume.level.toString());

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

    if (volume.level === 1) {
        player.volumeSvgTitle.textContent = T.mute;
        document.querySelector('.volume_hint.dimmed').classList.remove('active');
        document.querySelector('.volume_hint.muted').classList.remove('active');
    } else if (volume.level == 0) {
        player.volumeSvgTitle.textContent = T.unmute;
        document.querySelector('.volume_hint.dimmed').classList.remove('active');
        document.querySelector('.volume_hint.muted').classList.add('active');
    } else {
        player.volumeSvgTitle.textContent = T.mute;
        document.querySelector('.volume_hint.dimmed').classList.add('active');
        document.querySelector('.volume_hint.muted').classList.remove('active');
    }

    const beginAngle = -135;
    const arcAngle = volume.level * 270;

    const knobAngle = beginAngle + arcAngle;
    player.volumeButton.querySelector('path.knob').setAttribute('transform', `rotate(${knobAngle} 32 32)`);

    const activeD = volume.level > 0 ? segmentD(beginAngle, arcAngle) : '';
    player.volumeButton.querySelector('path.active_range').setAttribute('d', activeD);

    const inactiveD = volume.level < 1 ? segmentD(beginAngle + arcAngle, 270 - arcAngle) : '';
    player.volumeButton.querySelector('path.inactive_range').setAttribute('d', inactiveD);

    const percent = volume.level * 100;
    const percentFormatted = percent % 1 > 0.1 ? (Math.trunc(percent * 10) / 10) : Math.trunc(percent);
    player.volumeInput.setAttribute('aria-valuetext', `${T.volume} ${percentFormatted}%`);
    player.volumeInput.value = volume.level;

    if (restoreLevel === null) {
        delete volume.restoreLevel;
    } else {
        volume.restoreLevel = restoreLevel;
    }
}

player.container.addEventListener('keydown', event => {
    if (event.key === 'ArrowLeft') {
        event.preventDefault();
        const seekTo = Math.max(0, activeTrack.audio.currentTime - 5);
        togglePlayback(activeTrack, seekTo);
    } else if (event.key === 'ArrowRight') {
        event.preventDefault();
        const seekTo = Math.min(activeTrack.duration - 1, activeTrack.audio.currentTime + 5);
        togglePlayback(activeTrack, seekTo);
    }
});

player.playbackButton.addEventListener('click', () => {
    togglePlayback(activeTrack ?? firstTrack);
});

// Not available on a track player
if (player.nextTrackButton) {
    player.nextTrackButton.addEventListener('click', () => {
        if (activeTrack?.nextTrack) {
            togglePlayback(activeTrack.nextTrack);
        }
    });
}

// Not available on a track player
if (player.previousTrackButton) {
    player.previousTrackButton.addEventListener('click', () => {
        if (activeTrack?.previousTrack) {
            togglePlayback(activeTrack.previousTrack);
        }
    });
}

player.timeline.addEventListener('click', () => {
    const factor = (event.clientX - player.timeline.getBoundingClientRect().x) / player.timeline.getBoundingClientRect().width;
    const seekTo = factor * player.timelineInput.max;
    togglePlayback(activeTrack, seekTo);
    player.timelineInput.classList.add('focus_from_click');
    player.timelineInput.focus();
});

player.timelineInput.addEventListener('blur', () => {
    player.timelineInput.classList.remove('focus_from_click');
});

player.timelineInput.addEventListener('keydown', event => {
    if (event.key === ' ' || event.key === 'Enter') {
        event.preventDefault();
        togglePlayback(activeTrack);
    }
});

volume.container.addEventListener('wheel', event => {
    event.preventDefault();

    volume.level += event.deltaY * -0.0001;

    if (volume.level > 1) {
        volume.level = 1;
    } else if (volume.level < 0) {
        volume.level = 0;
    }

    updateVolume();
});

player.volumeButton.addEventListener('click', () => {
    if (volume.level > 0) {
        const restoreLevel = volume.level;
        volume.level = 0;
        updateVolume(restoreLevel);
    } else {
        volume.level = volume.restoreLevel ?? 1;
        updateVolume();
    }
});

player.volumeInput.addEventListener('input', () => {
    volume.level = parseFloat(player.volumeInput.valueAsNumber);
    updateVolume();
});

// This was observed to jump between 0 and 1 without a single step in between,
// hence we disable the default behavior and handle it ourselves
player.volumeInput.addEventListener('keydown', event => {
    if (event.key === 'ArrowLeft' || event.key === 'ArrowDown') {
        volume.level -= 0.02;
    } else if (event.key === 'ArrowRight' || event.key === 'ArrowUp') {
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
});

// This was observed to "scroll" between 0 and 1 without a single step in between,
// hence we disable the default behavior and let the event bubble up to our own handler
player.volumeInput.addEventListener('wheel', event => event.preventDefault());

let previousTrack = null;
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

    if (firstTrack === null) {
        firstTrack = track;
    }

    if (previousTrack !== null) {
        previousTrack.nextTrack = track;
        track.previousTrack = previousTrack;
    }

    previousTrack = track;

    audio.addEventListener('ended', event => {
        audio.currentTime = 0;
        container.classList.remove('active', 'playing');

        if (track.nextTrack) {
            togglePlayback(track.nextTrack);
        } else {
            activeTrack = null;
            player.container.classList.remove('active');
        }
    });

    audio.addEventListener('pause', event => {
        clearInterval(globalUpdatePlayHeadInterval);

        container.classList.remove('playing');
        player.playbackButton.replaceChildren(playIcon.content.cloneNode(true));

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
        player.playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));

        globalUpdatePlayHeadInterval = setInterval(() => updatePlayhead(track), 200);
        updatePlayhead(track);
        announcePlayhead(track);
    });

    audio.addEventListener('playing', event => {
        player.playbackButton.replaceChildren(pauseIcon.content.cloneNode(true));
    });

    audio.addEventListener('waiting', event => {
        // TODO: Eventually we could augment various screenreader labels here to
        //       indicate the loading state too
        player.playbackButton.replaceChildren(loadingIcon.content.cloneNode(true));
    });

    container.addEventListener('keydown', event => {
        if (event.key === 'ArrowLeft') {
            event.preventDefault();
            const seekTo = Math.max(0, track.audio.currentTime - 5);
            togglePlayback(track, seekTo);
        } else if (event.key === 'ArrowRight') {
            event.preventDefault();
            const seekTo = Math.min(track.duration - 1, track.audio.currentTime + 5);
            togglePlayback(track, seekTo);
        }
    });
}

mount(firstTrack);
