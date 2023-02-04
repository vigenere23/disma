import { Component, onMount } from "solid-js"
import Typed from "typed.js"
import { CliLine } from "../data/cli"

type LiveCLIProps = {
  lines: CliLine[]
}

function mapLinesToTypesJs(lines: CliLine[]): string {
  const addDelay = (text: string, delay?: number) => {
    if (delay !== undefined) {
      return `${text}^${delay}`
    }

    return text
  }

  const text = lines
    .map((line) => {
      switch (line.type) {
        case "input":
          return addDelay(line.text, line.msDelayAfter)
        case "output":
          return addDelay(`\`${line.text}\``, line.msDelayAfter)
      }
    })
    .join("\n")

  return text
}

export const LiveCLI: Component<LiveCLIProps> = ({ lines }) => {
  let element: HTMLDivElement | undefined
  const text = mapLinesToTypesJs(lines)
  const nbLines = text.split("\n").length + 1

  onMount(() => {
    new Typed(element as Element, {
      strings: [text],
      loop: true,
      typeSpeed: 20,
      showCursor: false,
      smartBackspace: false,
      backDelay: 5000,
    })
  })

  return (
    <div class="m-auto w-full max-w-2xl">
      <pre
        class="language-text m-0 !overflow-x-auto !overflow-y-hidden"
        style={{ height: `${nbLines * 24 + 6}px` }}
      >
        <code ref={element}></code>
      </pre>
    </div>
  )
}
