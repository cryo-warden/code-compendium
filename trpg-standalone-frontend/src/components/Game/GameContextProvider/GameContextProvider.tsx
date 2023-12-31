import { GameContext, useNewUIState } from "./GameContext";
import { ComponentChildren } from "preact";

const GameContextProvider = ({ children }: { children: ComponentChildren }) => {
  const uiState = useNewUIState();

  return (
    <GameContext.Provider value={uiState}>{children}</GameContext.Provider>
  );
};

export default GameContextProvider;
