import { IconTypes } from "solid-icons"
import { FaSolidFireFlameCurved } from "solid-icons/fa"

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
  icon: IconTypes
}

export const powers: Power[] = [
  {
    title: "Fast",
    description: "",
    icon: FaSolidFireFlameCurved,
  },
]
