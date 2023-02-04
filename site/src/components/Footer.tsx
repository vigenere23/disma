import { Component, For } from "solid-js"
import { footerLinks } from "../data/footer"

export const Footer: Component = () => {
  const year = new Date().getFullYear()

  return (
    <footer class="bg-dark-1 px-8 text-light-1">
      <div class="mx-auto flex h-32 w-full max-w-6xl flex-wrap items-center justify-between gap-4">
        <div class="[left]">Copyright © {year} Gabriel St-Pierre</div>
        <div class="[right] flex gap-4">
          <For each={footerLinks}>
            {(footerLink, index) => (
              <>
                {index() > 0 && <span>•</span>}
                <FooterLink
                  text={footerLink.text}
                  link={footerLink.link}
                ></FooterLink>
              </>
            )}
          </For>
        </div>
      </div>
    </footer>
  )
}

type FooterLinkProps = {
  link: string
  text: string
}

const FooterLink: Component<FooterLinkProps> = ({ link, text }) => {
  return (
    <a href={link} target="_blank">
      {text}
    </a>
  )
}
