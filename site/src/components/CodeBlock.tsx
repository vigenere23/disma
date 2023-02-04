import { Component, createEffect } from "solid-js"
import Prism from "prismjs"
import "../assets/prism-onedark.css"

type CodeBlockProps = {
  code: string
  language: string
}

export const CodeBlock: Component<CodeBlockProps> = ({ language, code }) => {
  createEffect(() => {
    Prism.highlightAll()
  })

  function trimCode(code: string) {
    return code.replace(/^\n+|(\n|\s)+$/g, "")
  }

  return (
    <pre class="!m-0 w-full">
      <code class={`language-${language}`}>{trimCode(code)}</code>
    </pre>
  )
}
