interface ServerViewProp {
  server?: number
}

export function ServerView({ server }: ServerViewProp) {
  return (
    <div className="w-full p-4">
      <h1 className="text-xl font-bold mb-4">Non-Scrollable Content</h1>
      {server !== undefined ? (<div>{server}</div>) : (<div></div>)}
      <p>This content does not scroll.</p>
    </div>

  )
}
