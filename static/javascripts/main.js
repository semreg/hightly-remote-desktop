window.onload = () => {
  document.querySelector('#inputIconEx2')
    .addEventListener('click', e => {
      e.target.select()
      document.execCommand('copy')
    })
}