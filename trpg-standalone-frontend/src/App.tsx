import { Dev } from "components/Game/Dev/Dev";
import Game from "components/Game/Game";
import GameContextProvider from "components/Game/GameContextProvider/GameContextProvider";
import "./App.css";

export const App = () => {
  return (
    <div class="App">
      <GameContextProvider>
        <div class="body">
          <Game />
        </div>
        <div class="header">cryo warden, work in progress site</div>
        <div class="left-gutter">
          <Dev />
        </div>
        <div class="right-gutter" />
      </GameContextProvider>
    </div>
  );
};
