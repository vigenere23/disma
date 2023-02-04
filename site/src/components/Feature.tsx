import { Component, For, JSXElement } from "solid-js"
import { Feature, features, Power, powers } from "../data/features"
import { CodeBlock } from "./CodeBlock"

export const FeatureSection: Component = () => {
  return (
    <section>
      <Features></Features>
      <Powers></Powers>
    </section>
  )
}

const SectionHeading: Component<{
  children: JSXElement
  className?: string
}> = ({ children, className }) => {
  return (
    <h2 class={`text-center text-5xl ${className ? className : ""}`}>
      {children}
    </h2>
  )
}

const Features: Component = () => {
  function isReversed(index: number): boolean {
    return index % 2 === 1
  }

  return (
    <div class="mx-auto flex w-full max-w-6xl flex-col items-center justify-center py-16 px-4 sm:px-8">
      <SectionHeading className="mb-16">Features</SectionHeading>
      <For each={features}>
        {(feature, index) => (
          <>
            {index() > 0 && (
              <div class="hidden h-[2px] w-full bg-light-3 md:block"></div>
            )}
            <FeatureItem
              feature={feature}
              reversed={isReversed(index())}
              className={`${index() + 1 < features.length ? "pb-16" : ""}`}
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
  className?: string
}

const FeatureItem: Component<FeatureItemProps> = ({
  feature,
  reversed,
  className,
}) => {
  return (
    <div
      class={`flex flex-col ${
        reversed ? "md:flex-row-reverse" : "md:flex-row"
      } ${
        className ? className : ""
      } w-full items-center justify-between pt-16`}
    >
      <div
        class={`md:grow md:basis-6/12 ${
          reversed
            ? "md:items-start md:text-left"
            : "md:items-end md:text-right"
        } mb-8 flex flex-col md:mt-8 md:px-16`}
      >
        <h3
          class={`mb-4 text-4xl ${reversed ? "md:mr-10" : "md:ml-10"}`}
          innerHTML={feature.title}
        ></h3>
        <p class="text-lg text-dark-4">{feature.description}</p>
      </div>
      <div class="relative w-full overflow-auto rounded-lg md:w-auto md:grow md:basis-5/12">
        <CodeBlock code={feature.code} language={feature.language}></CodeBlock>
      </div>
    </div>
  )
}

const Powers: Component = () => {
  return (
    <div class="py-16">
      <SectionHeading className="mb-32">Powers</SectionHeading>
      <div class="mx-auto grid w-full max-w-3xl grid-cols-2 gap-4 px-4 sm:gap-8 sm:px-8 md:grid-cols-3">
        <For each={powers}>
          {(power) => <PowerItem power={power}></PowerItem>}
        </For>
      </div>
    </div>
  )
}

type PowerItemProps = {
  power: Power
}

const PowerItem: Component<PowerItemProps> = ({ power }) => {
  return (
    <div class="flex cursor-pointer flex-col items-center rounded-md bg-dark-1 px-4 py-8 text-center text-light-1 shadow-2xl transition-all hover:opacity-90 hover:shadow-dark-4 sm:px-8">
      <div class="flex aspect-square w-full max-w-[90px] items-center justify-center rounded-full bg-light-1 text-dark-1">
        {power.icon()}
      </div>
      <h3 class="my-6 text-3xl">{power.title}</h3>
      <p>{power.description}</p>
    </div>
  )
}
