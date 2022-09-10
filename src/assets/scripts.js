// TODO: Revisit this at some point - assumes we only have one script, and
// generally a bit hacky/unclean in nature.
// Also the alt/text and markup for the play/pause icon could drift away
// from how we write/implement it in our statically generated code.
const rootPrefix = document.querySelector('script').getAttribute('src').replace('scripts.js', '');
const pauseIcon = `<img alt="Pause" src="${rootPrefix}pause.svg">`;
const playIcon = `<img alt="Play" src="${rootPrefix}play.svg">`;

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
    const controls = container.querySelector('.track_controls');
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
                controls.innerHTML = playIcon;
                
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
    controls.innerHTML = pauseIcon;

    if (seek) {
        audio.currentTime = seek * audio.duration;
    }

    audio.play();

    window.activeTrack = { a, audio, container, controls, svg, time }
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
                activeTrack.controls.innerHTML = pauseIcon;
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
                    activeTrack.controls.innerHTML = playIcon;
                    bigPlayButton.innerHTML = playIcon;
                }
            }
        } else {
            activeTrack.audio.pause();
            clearInterval(activeTrack.interval);
            activeTrack.audio.currentTime = 0;
            updatePlayhead(activeTrack, true);
            activeTrack.container.classList.remove('active', 'playing');
            activeTrack.controls.innerHTML = playIcon;
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
    svg.querySelector('stop:nth-child(1)').setAttribute('offset', `${factor * 100}%`);
    svg.querySelector('stop:nth-child(2)').setAttribute('offset', `${(factor + 0.0001) * 100}%`);
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