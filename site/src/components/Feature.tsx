import { Component, For } from "solid-js"
import { Feature, features } from "../data/features"
import { CodeBlock } from "./CodeBlock"

export const FeatureSection: Component = () => {
  function isReversed(index: number): boolean {
    return index % 2 === 1
  }

  return (
    <div class="my-12 mx-auto flex w-full max-w-6xl flex-col items-center justify-center px-4 sm:px-8">
      <For each={features}>
        {(feature, index) => (
          <>
            {index() > 0 && (
              <div class="hidden h-[2px] w-full bg-light-3 md:block"></div>
            )}
            <FeatureItem
              feature={feature}
              reversed={isReversed(index())}
            ></FeatureItem>
          </>
        )}
      </For>
    </div>
  )
}

type FeatureItemProps = {
  feature: Feature
  reversed: boolean
}

export const FeatureItem: Component<FeatureItemProps> = ({
  feature,
  reversed,
}) => {
  return (
    <div
      class={`flex flex-col ${
        reversed ? "md:flex-row-reverse" : "md:flex-row"
      } w-full items-center justify-between py-12 md:py-16`}
    >
      <div
        class={`md:grow md:basis-6/12 ${
          reversed
            ? "md:items-start md:text-left"
            : "md:items-end md:text-right"
        } mb-8 flex flex-col md:mt-8 md:px-16`}
      >
        <h2
          class={`mb-4 text-4xl ${reversed ? "md:mr-10" : "md:ml-10"}`}
          innerHTML={feature.title}
        ></h2>
        <p class="text-dark-4">{feature.description}</p>
      </div>
      <div class="relative w-full overflow-auto rounded-lg md:w-auto md:grow md:basis-5/12">
        <CodeBlock code={feature.code} language={feature.language}></CodeBlock>
      </div>
    </div>
  )
}
