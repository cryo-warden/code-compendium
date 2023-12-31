import Button from "components/UI/Button";
import { useUIState } from "../GameContextProvider/GameContext";

const Self = () => {
  const { selfEntityView, setFocus } = useUIState();
  if (selfEntityView == null) {
    return null;
  }

  return (
    <div>
      <Button onClick={() => setFocus(selfEntityView.id)}>
        {selfEntityView.name}
      </Button>
    </div>
  );
};

export default Self;
