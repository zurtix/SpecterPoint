import React, { useState, ChangeEvent, KeyboardEvent, useEffect, useRef } from "react";
import { Textarea } from "./textarea";

interface TerminalProps {
  id: string,
  type: string,
  history: string[],
  commands: string[],
  onHistory: (id: string, type: string, history: string[]) => void,
  onCommand: (id: string, type: string, command: string[]) => void,
  onExit: (id: string, type: string) => void
}

const Terminal: React.FC<TerminalProps> = ({ id, type, history, commands, onHistory, onCommand, onExit }) => {
  const [input, setInput] = useState<string>("")
  const textAreaRef = useRef<HTMLTextAreaElement>(null)
  const [pos, setPos] = useState<number>(1)

  useEffect(() => {
    if (textAreaRef.current) {
      textAreaRef.current.style.height = "auto"
      textAreaRef.current.style.height = `${textAreaRef.current.scrollHeight}px`
    }
  }, [input])

  const handleInputChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setInput(e.currentTarget.value)
  }

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      processCommand(input.trim())
      setInput("")
      setPos(0)
    }

    if (e.key == "ArrowUp") {
      e.preventDefault()
      if (pos <= commands.length) {
        setInput(commands[commands.length - pos])
        setPos(v => v + 1)
      }
    }

    if (e.key == "ArrowDown") {
      e.preventDefault()
      if (pos > 1) {
        setInput(commands[commands.length - pos])
        setPos(v => v - 1)
      } else {
        setInput("")
      }
    }
  }

  const processCommand = (command: string) => {
    onCommand(id, type, [...commands.slice(-100), command])
    let response: string;
    switch (command.toLowerCase()) {
      case "help":
        response = "Available commands: help, echo, clear"
        break
      case "exit":
        onExit(id, type)
        return
      case "clear":
        onHistory(id, type, [])
        return
      case command.startsWith("echo") && command:
        response = command.slice(5)
        break
      default:
        response = `command not found: ${command}`
    }
    onHistory(id, type, [...history, `> ${command}`, response])
  }

  return (
    <div className="w-full h-full text-sm p-2"
      onMouseEnter={() => textAreaRef.current?.focus()}
      onClick={() => textAreaRef.current?.focus()}>
      <div className="break-words">
        {history.map((item, index) => (
          <div className="overflow-clip" key={index}>{item}</div>
        ))}
      </div>
      <div className="flex">
        <span className="text-green-600">{'> '}</span>
        <Textarea
          ref={textAreaRef}
          className="h-auto resize-none overflow-hidden outline-none w-full break-words"
          rows={1}
          value={input}
          onChange={handleInputChange}
          onKeyDownCapture={handleKeyDown}
          autoFocus
        />
      </div>
    </div>
  )
}

export default Terminal
