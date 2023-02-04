import {
  FaSolidBolt,
  FaSolidBriefcase,
  FaSolidCodeFork,
  FaSolidGem,
  FaSolidLightbulb,
} from "solid-icons/fa"
import { ImCheckmark } from "solid-icons/im"
import { JSXElement } from "solid-js"

export type Feature = {
  title: string
  description: string
  code: string
  language: string
}

export const features: Feature[] = [
  {
    title: "Design once, repeat infinitely",
    description:
      "Loop over repeated configs instead of copy-pasting by leveraging the power of handlebars templating.",
    code: `
categories:
  items:
  {{!-- loop over all teams --}}
  {{#each teams}}
    - name: Team-{{this}}
  {{/each}}

channels:
  items:
  {{#each teams}}
    - category: Team-{{this}}
      name: general
  {{/each}}
    `,
    language: "handlebars",
  },
  {
    title: "See changes before they happen",
    description:
      "Every command show the complete diffs before applying, making sure you know exactly what changes will be made.",
    code: `● 🔄 Updating Role student with diffs:
   permissions:
-    CONNECT
   is_mentionable:
-    false
+    true
   color:
-    2785e8
+    34f5e2`,
    language: "diff",
  },
  {
    title: "Start slowly, adopt incrementally",
    description:
      "Skip existing items that are not yet present in the config to integrate them one step at a time.",
    code: `
roles:
  items:
  - name: Staff
    show_in_sidebar: true
    mentionable: true
    permissions: [ADMINISTRATOR]

  extra_items:
    # Ignore other roles
    strategy: KEEP
    `,
    language: "yaml",
  },
  {
    title: "Explicitize the implicit",
    description:
      "Clearly define if channel's permissions should always be synched with its category's.",
    code: `
categories:
  items:
  - name: 'Team-23'
    permissions_overwrites:
    - role: team23
      allow: [VIEW_CHANNEL]

channels:
  items:
  - category: 'Team-23'
    name: general
    permissions_overwrites:
      # Force permissions sync
      strategy: FROM_CATEGORY
    `,
    language: "yaml",
  },
]

export type Power = {
  title: string
  description: string
  icon: () => JSXElement
}

export const powers: Power[] = [
  {
    title: "Simple",
    description: "Just a small YAML file, with schema validation",
    icon: () => <FaSolidLightbulb size={36}></FaSolidLightbulb>,
  },
  {
    title: "Fast",
    description: "Builded with Rust, one of the fastest language",
    icon: () => <FaSolidBolt size={36}></FaSolidBolt>,
  },
  {
    title: "Portable",
    description: "Easily install with curl, no Cargo needed",
    icon: () => <FaSolidBriefcase size={36}></FaSolidBriefcase>,
  },
  {
    title: "Ready",
    description: "Already-made templates for the most common configs",
    icon: () => <ImCheckmark size={36}></ImCheckmark>,
  },
  {
    title: "Flexible",
    description: "Create your own implementation with the core library",
    icon: () => <FaSolidCodeFork size={36}></FaSolidCodeFork>,
  },
  {
    title: "Complete",
    description: "Manage everything, from roles  all the way to channels",
    icon: () => <FaSolidGem size={36}></FaSolidGem>,
  },
]
