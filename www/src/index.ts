import './style.css';

const rust = import('snake-wasm');

function get_greeting_div(): HTMLDivElement {
    const div: HTMLDivElement = document.createElement('div');
    const btn: HTMLButtonElement = document.createElement('button');

    btn.innerHTML = 'GREET';
    btn.classList.add('browny');
    btn.onclick = () => rust
        .then(m => m.greet('Typescript'))
        .catch(console.error);
    div.appendChild(btn);

    return div;
}

document.body.appendChild(get_greeting_div());
