import "./Others.css";
import Button from "components/UI/Button";
import { useUIState, GameUIState } from "../GameContextProvider/GameContext";

type EntityViewProps = {
  entityView: GameUIState["entityViews"][number];
};

const EntityView = ({ entityView }: EntityViewProps) => {
  const { setFocus } = useUIState();
  return (
    <div class="EntityView">
      <Button onClick={() => setFocus(entityView.id)}>{entityView.name}</Button>
    </div>
  );
};

const Others = () => {
  const { entityViews, selfEntityView } = useUIState();
  return (
    <div class="Others">
      {Object.values(entityViews).map(
        (entityView) =>
          (selfEntityView == null || selfEntityView.id != entityView.id) && (
            <EntityView key={entityView.id} entityView={entityView} />
          )
      )}
    </div>
  );
};

export default Others;
