/**
 * Copyright 2025 Magic Mount-rs Authors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import type { JSX } from "solid-js";

type BaseProps = JSX.HTMLAttributes<HTMLElement> & {
  [key: `attr:${string}`]: string | undefined;
  [key: `on:${string}`]: any;
  [key: `prop:${string}`]: unknown;
};

interface MdDialogProps extends BaseProps {
  open?: boolean;
  onclose?: (e: Event) => void;
}

interface MdTextFieldProps extends BaseProps {
  "label"?: string;
  "value"?: string;
  "error"?: boolean;
  "supporting-text"?: string;
  "disabled"?: boolean;
  "type"?: string;
  "onInput"?: (e: InputEvent) => void;
}

interface MdButtonProps extends BaseProps {
  disabled?: boolean;
  type?: string;
  href?: string;
  target?: string;
  onClick?: (e: MouseEvent) => void;
}

interface MdIconButtonProps extends BaseProps {
  disabled?: boolean;
  type?: string;
  href?: string;
  target?: string;
  title?: string;
  onClick?: (e: MouseEvent) => void;
}

interface MdListItemProps extends BaseProps {
  type?: string;
  href?: string;
  target?: string;
  disabled?: boolean;
}

declare module "solid-js" {
  namespace JSX {
    interface IntrinsicElements {
      "md-icon": BaseProps;
      "md-icon-button": MdIconButtonProps;
      "md-filled-tonal-icon-button": MdIconButtonProps;
      "md-filled-button": MdButtonProps;
      "md-text-button": MdButtonProps;
      "md-filled-tonal-button": MdButtonProps;
      "md-outlined-text-field": MdTextFieldProps;
      "md-dialog": MdDialogProps;
      "md-ripple": BaseProps;
      "md-list": BaseProps;
      "md-list-item": MdListItemProps;
      "md-switch": BaseProps & { selected?: boolean };
      "md-divider": BaseProps;
    }
  }
}
