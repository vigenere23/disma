import { Component } from "solid-js"
import { LinkButton } from "./Button"
import { Logo } from "./Logo"
import { FaBrandsGithub } from "solid-icons/fa"

export const Header: Component = () => {
  return (
    <header class="flex h-16 w-full items-center justify-between border-b-2 border-solid border-black px-4">
      <div class="[left]">
        <Logo></Logo>
      </div>
      <div class="[right] flex shrink-0 gap-x-3">
        <LinkButton
          text="Repo"
          link="https://github.com/vigenere23/disma"
          icon={FaBrandsGithub}
        ></LinkButton>
      </div>
    </header>
  )
}
