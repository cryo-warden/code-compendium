import "./Game.css";
import Log from "./Log/Log";
import Focus from "./Focus/Focus";
import Self from "./Self/Self";
import Others from "./Others/Others";

const Game = () => (
  <div class="Game">
    <div class="log">
      <Log />
    </div>
    <div class="focus">
      <Focus />
    </div>
    <div class="self">
      <Self />
    </div>
    <div class="others">
      <Others />
    </div>
  </div>
);

export default Game;
