import { createContext } from "preact";
import { createDemoWorldContainer } from "demo/DemoWorld";
import { EntityView, Message } from "demo/OutputEvent";
import { Action } from "demo/InputEvent";
import { createEngine } from "GameEngine/Engine";
import { EntityId } from "GameEngine/Entity";
import { useContext, useEffect, useState } from "preact/hooks";

export type GameUIState = {
  messages: Message[];
  selfEntityView: EntityView | null;
  entityViews: Record<EntityId, EntityView>;
  focusEntityId: EntityId | null;
  setFocus: (entityId: EntityId) => void;
  executeAction: (action: Action) => void;
};

const emptyUIState: GameUIState = {
  messages: [],
  selfEntityView: null,
  entityViews: {},
  focusEntityId: null,
  setFocus: () => {},
  executeAction: () => {},
};

export const GameContext = createContext<GameUIState>(emptyUIState);

export const useNewUIState = (): GameUIState => {
  const [state, setState] = useState(emptyUIState);
  useEffect(() => {
    const engine = createEngine();

    // TODO Create UI to load a saved world.
    const demoWorldContainer = createDemoWorldContainer();
    engine.setWorld(demoWorldContainer.world);

    const logSubscription = demoWorldContainer.subscribe(
      "logMessage",
      (event) => {
        setState((state) => ({
          ...state,
          messages: [...state.messages, event.message],
        }));
      }
    );

    const setSelfSubscription = demoWorldContainer.subscribe(
      "observeSelf",
      (event) => {
        setState((state) => ({
          ...state,
          selfEntityView: event.entityView,
        }));
      }
    );

    const gainViewSubscription = demoWorldContainer.subscribe(
      "observeEntity",
      (event) => {
        setState((state) => ({
          ...state,
          entityViews: {
            ...state.entityViews,
            [event.entityView.id]: event.entityView,
          },
        }));
      }
    );

    const loseViewSubscription = demoWorldContainer.subscribe(
      "unobserveEntity",
      (event) => {
        setState((state) => {
          const { [event.entityId]: _, ...entityViews } = state.entityViews;
          return { ...state, entityViews };
        });
      }
    );

    engine.start();

    setState({
      ...emptyUIState,
      setFocus: (entityId) => {
        setState((state) => ({ ...state, focusEntityId: entityId }));
      },
      executeAction: demoWorldContainer.executeAction,
    });

    return () => {
      logSubscription.cancel();
      setSelfSubscription.cancel();
      gainViewSubscription.cancel();
      loseViewSubscription.cancel();
      engine.dispose();
      demoWorldContainer.dispose();
    };
  }, [setState]);

  return state;
};

export const useUIState = (): GameUIState => useContext(GameContext);
