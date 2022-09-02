// TODO: Revisit this at some point - assumes we only have one script, and
// generally a bit hacky/unclean in nature.
// Also the alt/text and markup for the play/pause icon could drift away
// from how we write/implement it in our statically generated code.
const rootPrefix = document.querySelector('script').getAttribute('src').replace('scripts.js', '');
const pauseIcon = `<img alt="Pause" src="${rootPrefix}pause.svg">`;
const playIcon = `<img alt="Play" src="${rootPrefix}play.svg">`;

window.playing = null;

function beginPlayback(container) {
    const a = container.querySelector('a');
    const audio = container.querySelector('audio');
    const controls = container.querySelector('.track_controls');
    const svg = container.querySelector('svg');

    container.classList.add('playing');
    
    if (!audio.dataset.endedListenerAdded) {
        audio.dataset.endedListenerAdded = true;
        
        audio.addEventListener('ended', event => {
            if (window.playing && window.playing.audio === audio) {
                audio.currentTime = 0;
                updatePlayhead(audio, svg);
                clearInterval(window.playing.interval);
                container.classList.remove('playing');
                document.querySelector('.track_play').innerHTML = playIcon;
                controls.innerHTML = playIcon;
                
                const nextContainer = container.nextElementSibling;
                if (nextContainer && nextContainer.classList.contains('track')) {
                    beginPlayback(nextContainer);
                } else {
                    window.playing = null;
                }
            }
        });
    }
    
    audio.play();

    document.querySelector('.track_play').innerHTML = pauseIcon;
    controls.innerHTML = pauseIcon;

    const interval = setInterval(() => updatePlayhead(audio, svg), 200);
    
    window.playing = { a, audio, container, controls, interval, svg };
}

function share(a) {
    const notify = message => {
        const prevNotification = a.parentElement.querySelector('.share_notification');
        if (prevNotification) prevNotification.remove();
            
        const newNotification = document.createElement('span');
        newNotification.classList.add('share_notification');
        newNotification.innerHTML = message;
        a.parentElement.appendChild(newNotification);
    };
    
    navigator.clipboard
        .writeText(window.location.href)
        .then(() => notify(`${window.location.href} has been copied to your clipboard`))
        .catch(err => notify(`Failed to copy ${window.location.href} to your clipboard (${err})`));
};

function updatePlayhead(audio, svg) {
    const factor = audio.currentTime / audio.duration;
    svg.querySelector('stop:nth-child(1)').setAttribute('offset', `${factor * 100}%`);
    svg.querySelector('stop:nth-child(2)').setAttribute('offset', `${(factor + 0.0001) * 100}%`);
}

window.addEventListener('DOMContentLoaded', event => {
    const shareLink = document.querySelector('.share_link');

    if (shareLink && navigator.clipboard) {
        shareLink.classList.remove('disabled');
        shareLink.removeAttribute('title');
    }
});

// TODO: Clean/DRY up logic globally
document.body.addEventListener('click', event => {
    if (event.target.classList.contains('track_play')) {
        if (window.playing) {
            if (window.playing.audio.paused) {
                window.playing.container.classList.add('playing');
                document.querySelector('.track_play').innerHTML = pauseIcon;
                window.playing.controls.innerHTML = pauseIcon;
                window.playing.audio.play();
                window.playing.interval = setInterval(() => updatePlayhead(window.playing.audio, window.playing.svg), 200);
            } else {
                window.playing.audio.pause();
                updatePlayhead(window.playing.audio, window.playing.svg);
                clearInterval(window.playing.interval);
                window.playing.container.classList.remove('playing');
                document.querySelector('.track_play').innerHTML = playIcon;
                window.playing.controls.innerHTML = playIcon;
            }
        } else {
            const firstContainer = event.target.parentElement.parentElement.nextElementSibling;
            beginPlayback(firstContainer);
        }
    } else if (event.target.classList.contains('track_title_wrapper')) {
        event.preventDefault();
        
        const container = event.target.parentElement;
        const audio = container.querySelector('audio');
        const controls = container.querySelector('.track_controls');
        
        if (window.playing && window.playing.audio !== audio) {
            window.playing.audio.pause();
            window.playing.audio.currentTime = 0;
            updatePlayhead(window.playing.audio, window.playing.svg);
            clearInterval(window.playing.interval);
            window.playing.container.classList.remove('playing');
            document.querySelector('.track_play').innerHTML = playIcon;
            window.playing.controls.innerHTML = playIcon;
            window.playing = null;
        }
        
        if (audio.paused) {    
            beginPlayback(container);
        } else {
            audio.pause();
            container.classList.remove('playing');
            document.querySelector('.track_play').innerHTML = playIcon;
            controls.innerHTML = playIcon;
        }
    } else if (event.target.classList.contains('waveform')) {
        const container = event.target.parentElement.parentElement;
        const audio = container.querySelector('audio');
        const svg = event.target;

        const factor = (event.clientX - event.target.getBoundingClientRect().x) / event.target.getBoundingClientRect().width;
        
        if (window.playing && window.playing.audio !== audio) {
            // TODO: DRY
            window.playing.audio.pause();
            window.playing.audio.currentTime = 0;
            updatePlayhead(window.playing.audio, window.playing.svg);
            clearInterval(window.playing.interval);
            window.playing.container.classList.remove('playing');
            document.querySelector('.track_play').innerHTML = playIcon;
            window.playing.controls.innerHTML = playIcon;
            window.playing = null;
        }
        
        if (window.playing) {
            audio.currentTime = factor * audio.duration;
            updatePlayhead(audio, svg);
        } else {
            audio.currentTime = factor * audio.duration;
            beginPlayback(container);
        }
    } else if (event.target.classList.contains('share_link') && !event.target.classList.contains('disabled')) {
        event.preventDefault();
        share(event.target);
    }
});