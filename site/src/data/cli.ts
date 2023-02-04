export type CliLine = {
  type: "input" | "output"
  text: string
  msDelayAfter?: number
}

export const cliLines: CliLine[] = [
  {
    type: "input",
    text: ">  disma apply -g 123989012389 -i config.yaml",
    msDelayAfter: 1000,
  },
  {
    type: "output",
    text: "➜ 🔎 Looking for changes...",
    msDelayAfter: 2000,
  },
  {
    type: "output",
    text: "➜ 📜 Found the following changes :",
  },
  {
    type: "output",
    text: "\n● 🆕  Adding Role student",
  },
  {
    type: "output",
    text: `● 🔄  Updating Role staff with diffs:
    permissions:
+     VIEW_CHANNEL`,
  },
  {
    type: "output",
    text: "● 🗑️  Removing Role team-26",
  },
  {
    type: "input",
    text: "`\n➜ ❔ Ready to apply? (y/N)`^2000 y",
    msDelayAfter: 1000,
  },
  {
    type: "output",
    text: "➜ 🚀 Applying changes...",
    msDelayAfter: 500,
  },
  {
    type: "output",
    text: "\n● 🆕  Adding Role student...Done",
    msDelayAfter: 500,
  },
  {
    type: "output",
    text: "● 🔄  Updating Role staff...Done",
    msDelayAfter: 500,
  },
  {
    type: "output",
    text: "● 🗑️  Removing Role team-26...Done",
    msDelayAfter: 500,
  },
  {
    type: "output",
    text: ">",
  },
]
