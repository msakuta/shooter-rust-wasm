import enemy from "../assets/enemy.png";
import player from "../assets/player.png";
import bullet from "../assets/bullet.png";
import ebullet from "../assets/ebullet.png";
import boss from "../assets/boss.png";
import missile from "../assets/missile.png";
import explode from "../assets/explode.png";
import explode2 from "../assets/explode2.png";
import trail from "../assets/trail.png";
import beam from "../assets/beam.png";
import back from "../assets/back2.jpg";
import power from "../assets/power.png";
import power2 from "../assets/power2.png";
import sphere from "../assets/sphere.png";
import weapons from "../assets/weapons.png";

const bulletsElement = document.getElementById("bullets");
bulletsElement.style.backgroundImage = `url(${sphere})`;

const bulletsInElement = document.getElementById("bulletsIn");
bulletsInElement.style.backgroundImage = `url(${weapons})`;

const missilesElement = document.getElementById("missiles");
missilesElement.style.backgroundImage = `url(${sphere})`;

const missilesInElement = document.getElementById("missilesIn");
missilesInElement.style.backgroundImage = `url(${weapons})`;
missilesInElement.style.backgroundPosition = `-96px 0px`;

const lightElement = document.getElementById("light");
lightElement.style.backgroundImage = `url(${sphere})`;

const lightInElement = document.getElementById("lightIn");
lightInElement.style.backgroundImage = `url(${weapons})`;
lightInElement.style.backgroundPosition = `-64px 0px`;

const lightningElement = document.getElementById("lightning");
lightningElement.style.backgroundImage = `url(${sphere})`;

const lightningInElement = document.getElementById("lightningIn");
lightningInElement.style.backgroundImage = `url(${weapons})`;
lightningInElement.style.backgroundPosition = `-128px 0px`;

function updateWeapons(weapon){
  const dark = "grayscale() brightness(0.75)";
  bulletsElement.style.filter = dark;
  missilesElement.style.filter = dark;
  lightElement.style.filter = dark;
  lightningElement.style.filter = dark;
  switch (weapon) {
    case "Bullet": bulletsElement.style.filter = `hue-rotate(90deg) brightness(1.2)`; break;
    case "Light": lightElement.style.filter = "grayscale() brightness(1.45)"; break;
    case "Missile": missilesElement.style.filter = " brightness(1.2)"; break;
    case "Lightning": lightningElement.style.filter = "hue-rotate(30deg) brightness(1.2)"; break;
  }
}

updateWeapons("Bullet");

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
      ["ebullet", ebullet],
      ["boss", boss],
      ["missile", missile],
      ["explode", explode],
      ["explode2", explode2],
      ["trail", trail],
      ["beam", beam],
      ["back", back],
      ["power", power],
      ["power2", power2],
      ["sphere", sphere],
      ["weapons", weapons],
    ];
    let state = new module.ShooterState(image_assets);

    state.start();

    document.addEventListener("keydown", (event) => {
      const weapon = state.key_down(event);
      updateWeapons(weapon);
    });
    document.addEventListener("keyup", (event) => state.key_up(event));

    const restartButton = document.getElementById("restart");
    restartButton.addEventListener("click", () => state.restart());

    function render() {
      state.render();
      requestAnimationFrame(render);
    }
    requestAnimationFrame(render);
  });
