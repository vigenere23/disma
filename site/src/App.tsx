import type { Component } from "solid-js"
import { FeatureSection } from "./components/Feature"
import { Footer } from "./components/Footer"
import { Header } from "./components/Header"
import { Hero } from "./components/Hero"
import { Intro } from "./components/Intro"

export const App: Component = () => {
  return (
    <div>
      <Header></Header>
      <Hero></Hero>
      <main class="py-16">
        <Intro></Intro>
        <FeatureSection></FeatureSection>
      </main>
      <Footer></Footer>
    </div>
  )
}
