import { useUIState } from "../GameContextProvider/GameContext";
import "./Dev.css";

export const Dev = () => {
  const { selfEntityView } = useUIState();

  // WIP Only display this after a method is called on a global `dev` object.
  // WIP Hook into raw engine data, and not only the entity views.
  return (
    <div class="Dev" style={{ whiteSpace: "pre" }}>
      {JSON.stringify(selfEntityView, null, 2)}
    </div>
  );
};
