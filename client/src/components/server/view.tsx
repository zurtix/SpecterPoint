import { Server } from "@/components/server/types"

export function ServerView({ server }: { server?: Server }) {
  return (
    (server &&
      <div className="w-full p-4">
        <p>{server.id}</p>
      </div>)

  )
}
