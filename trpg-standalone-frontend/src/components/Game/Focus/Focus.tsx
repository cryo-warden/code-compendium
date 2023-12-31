import "./Focus.css";
import { useUIState } from "../GameContextProvider/GameContext";
import Button from "components/UI/Button";

// WIP Add action buttons.
const Focus = () => {
  const { entityViews, focusEntityId, executeAction } = useUIState();
  if (focusEntityId == null) {
    return null;
  }

  const focusedEntityView = entityViews[focusEntityId];
  if (focusedEntityView == null) {
    return null;
  }

  return (
    <div class="Focus">
      <div class="name">{focusedEntityView.name}</div>
      <div class="description">{focusedEntityView.description}</div>
      <div>
        {focusedEntityView.interactions.map((interaction) => (
          <Button
            key={interaction.label}
            onClick={() => executeAction(interaction.action)}
          >
            {interaction.label}
          </Button>
        ))}
      </div>
    </div>
  );
};

export default Focus;
