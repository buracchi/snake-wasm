import './style.css';

import { SnakeGameManager } from './snake-game/snake-game-manager';

const gameManager = new SnakeGameManager();
document.body.appendChild(gameManager.root);
gameManager.run()
