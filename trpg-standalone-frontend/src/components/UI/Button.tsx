import { ComponentChildren } from "preact";

import "./styles.css";

const Button = ({
  children,
  onClick,
}: {
  children: ComponentChildren;
  onClick: () => void;
}) => (
  <button class="Button" onClick={onClick}>
    {children}
  </button>
);

export default Button;
