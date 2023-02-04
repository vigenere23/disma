import { Component } from "solid-js"
import { cliLines } from "../data/cli"
import { LiveCLI } from "./LiveCLI"

export const Intro: Component = () => {
  return (
    <div class="py-16 px-4 md:px-8">
      <p class="mx-auto mb-16 max-w-2xl text-center text-lg font-bold leading-relaxed text-dark-4">
        No longer worry about your Discord's setup. Offered as both a Rust
        library and a command line tool, Disma let's you manage you guild
        quickly and easily with a simple configuration format.
      </p>
      <LiveCLI lines={cliLines}></LiveCLI>
    </div>
  )
}
