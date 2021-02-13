window.addEventListener('DOMContentLoaded', event => {
    const shareLink = document.querySelector('.share');

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
    } else if (event.target.classList.contains('share') && !event.target.classList.contains('disabled')) {
        event.preventDefault();
        
        // TODO: Visual confirmation along the lines of "Link <url-here> has been copied to your clipboard"
        
        navigator.clipboard
            .writeText(event.target.dataset.url)
            .catch(err => alert(`Failed to copy: ${err}`));
    } 
});