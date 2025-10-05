import { cva } from "class-variance-authority"
import { BookIcon, GithubIcon, MessagesSquareIcon } from "lucide-react"
import { Logo } from "./logo"
import { Wrapper } from "./wrapper"

const headerLinkVariants = cva([
  "group flex cursor-pointer items-center gap-1.5",
  "*:last:underline-offset-2 *:last:group-hover:underline [&_svg]:size-4 [&_svg]:shrink-0",
])

export function Header() {
  return (
    <Wrapper className="flex flex-col items-center justify-center gap-4">
    

      <h1 className="text-center font-extrabold">
      Pizza Charity Limited
      </h1>

      <div className="flex items-center gap-6">
        <a
          href="https://github.com/AbuTuraab"
          target="_blank"
          rel="noopener noreferrer"
          className={headerLinkVariants()}
        >
          <GithubIcon />
          <span>GitHub</span>
        </a>

      

     
      </div>
    </Wrapper>
  )
}
