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

window.addEventListener('DOMContentLoaded', event => {
    const shareLink = document.querySelector('.share_link');

    if (shareLink && navigator.clipboard) {
        shareLink.classList.remove('disabled');
        shareLink.removeAttribute('title');
    }
});

document.body.addEventListener('click', event => {
    if (event.target.classList.contains('track_number')) {
        event.preventDefault();
        
        const a = event.target;
        const audio = a.parentElement.querySelector('audio');
        
        for (const iteratedAudio of document.querySelectorAll('audio')) {
            if (iteratedAudio !== audio && !iteratedAudio.paused) {
                iteratedAudio.pause();
                iteratedAudio.parentElement.querySelector('a.play').innerHTML = '▶️';
            }
        }
        
        if (audio.paused) {
            audio.play();
            a.innerHTML = 'Pa';
        } else {
            audio.pause();
            a.innerHTML = 'Pl';
        }
    } else if (event.target.classList.contains('share_link') && !event.target.classList.contains('disabled')) {
        event.preventDefault();
        share(event.target);
    } 
});