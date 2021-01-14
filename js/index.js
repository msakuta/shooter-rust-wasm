import enemy from "../assets/enemy.png";
import player from "../assets/player.png";
import bullet from "../assets/bullet.png";

// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../pkg');

rust
  .catch(console.error)
  .then((module) => {
    let image_assets = [
      ["enemy", enemy],
      ["player", player],
      ["bullet", bullet],
    ];
    module.start(image_assets);
  });
