window.playing = null;

const share = a => {
    const notify = message => {
        const prevNotification = a.parentElement.querySelector('.share_notification');
        if (prevNotification) prevNotification.remove();
            
        const newNotification = document.createElement('span');
        newNotification.classList.add('share_notification');
        newNotification.innerHTML = message;
        a.parentElement.appendChild(newNotification);
    };
    
    navigator.clipboard
        .writeText(a.dataset.url)
        .then(() => notify(`${window.location.href} has been copied to your clipboard`))
        .catch(err => notify(`Failed to copy ${window.location.href} to your clipboard (${err})`));
};

const updatePlayhead = (audio, svg) => {
    const factor = audio.currentTime / audio.duration;
    
    svg.querySelector('stop:nth-child(1)').setAttribute('offset', `${factor * 100}%`);
    svg.querySelector('stop:nth-child(2)').setAttribute('offset', `${(factor + 0.0001) * 100}%`);
};

const beginPlayback = (a, audio, svg) => {
    a.classList.add('playing');
    
    if (!audio.dataset.endedListenerAdded) {
        audio.dataset.endedListenerAdded = true;
        
        audio.addEventListener('ended', event => {
            if (window.playing && window.playing.audio === audio) {
                audio.currentTime = 0;
                updatePlayhead(audio, svg);
                clearInterval(window.playing.interval);
                a.classList.remove('playing');
                
                const nextTrack = audio.parentElement.nextElementSibling;
                if (nextTrack && nextTrack.classList.contains('track_title_wrapper')) {
                    const nextA = nextTrack.querySelector('a');
                    const nextAudio = nextTrack.nextElementSibling.querySelector('audio');
                    const nextSvg = nextTrack.nextElementSibling.querySelector('svg');
                    
                    beginPlayback(nextA, nextAudio, nextSvg);
                } else {
                    window.playing = null;
                }
            }
        });
    }
    
    audio.play();
    
    window.playing = {
        a,
        audio,
        interval: setInterval(() => updatePlayhead(audio, svg), 200),
        svg
    };
};

window.addEventListener('DOMContentLoaded', event => {
    const shareLink = document.querySelector('.share_link');

    if (shareLink && navigator.clipboard) {
        shareLink.classList.remove('disabled');
        shareLink.removeAttribute('title');
    }
});

document.body.addEventListener('click', event => {
    if (event.target.classList.contains('track_title')) {
        event.preventDefault();
        
        const a = event.target;
        const audio = event.target.parentElement.nextElementSibling.querySelector('audio');
        const svg = event.target.parentElement.nextElementSibling.querySelector('svg');
        
        if (window.playing && window.playing.audio !== audio) {
            window.playing.audio.pause();
            window.playing.audio.currentTime = 0;
            updatePlayhead(window.playing.audio, window.playing.svg);
            clearInterval(window.playing.interval);
            window.playing.a.classList.remove('playing');
            window.playing = null;
        }
        
        if (audio.paused) {    
            beginPlayback(a, audio, svg);
        } else {
            audio.pause();
            a.classList.remove('playing');
        }
    } else if (event.target.classList.contains('waveform')) {
        const a = event.target.parentElement.previousElementSibling.querySelector('a');
        const audio = event.target.parentElement.querySelector('audio');
        const factor = (event.clientX - event.target.getBoundingClientRect().x) / event.target.getBoundingClientRect().width;
        const svg = event.target;
        
        if (window.playing && window.playing.audio !== audio) {
            // TODO: DRY
            window.playing.audio.pause();
            window.playing.audio.currentTime = 0;
            updatePlayhead(window.playing.audio, window.playing.svg);
            clearInterval(window.playing.interval);
            window.playing.a.classList.remove('playing');
            window.playing = null;
        }
        
        if (window.playing) {
            audio.currentTime = factor * audio.duration;
            updatePlayhead(audio, svg);    
        } else {
            audio.currentTime = factor * audio.duration;
            beginPlayback(a, audio, svg);
        }
    } else if (event.target.classList.contains('share_link') && !event.target.classList.contains('disabled')) {
        event.preventDefault();
        share(event.target);
    }
});