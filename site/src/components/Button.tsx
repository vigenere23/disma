import { IconTypes } from "solid-icons"
import { Component, Show } from "solid-js"
import { Dynamic } from "solid-js/web"

type LinkButtonProps = {
  icon?: IconTypes
  text: string
  link: string
}

export const LinkButton: Component<LinkButtonProps> = ({
  icon,
  text,
  link,
}) => {
  return (
    <a
      href={link}
      target="_blank"
      class="block h-10 rounded-[20px] bg-dark-1 py-2 px-4 text-light-1 hover:bg-dark-3 focus:bg-dark-3 active:bg-dark-4 dark:bg-light-1 dark:text-dark-1 dark:hover:bg-light-3 dark:focus:bg-light-3 dark:active:bg-light-4"
    >
      <Show when={!!icon}>
        <Dynamic component={icon} class="mr-2 inline align-sub"></Dynamic>
      </Show>
      <span>{text}</span>
    </a>
  )
}
