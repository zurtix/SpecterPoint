import { useNavigate } from '@tanstack/react-router'
import { useForm } from "@tanstack/react-form"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import {
  FormItem
} from "@/components/ui/tanstack-form"
import { useToast } from "@/components/ui/use-toast"
import { Separator } from "@/components/ui/separator"
import { Input } from "@/components/ui/input"
import { invoke } from '@tauri-apps/api/tauri'
import { zodValidator } from '@tanstack/zod-form-adapter'

export default function LoginForm() {
  const navigate = useNavigate()
  const { toast } = useToast()
  const form = useForm({
    defaultValues: {
      username: "",
      password: "",
      key: ""
    },
    onSubmit: async ({ value }) => {
      invoke("login", value).then((_) =>
        navigate({
          to: "/targets"
        })
      ).catch((err: string) =>
        toast({
          variant: "destructive",
          title: err,
          description: "Review username, password, and encryption key",
        })
      )
    },
    validatorAdapter: zodValidator()
  })

  return (
    <div className="w-full">
      <form onSubmit={(e) => {
        e.preventDefault()
        e.stopPropagation()
        form.handleSubmit()
      }}>
        <div className="flex flex-col gap-2 w-full">
          <form.Field
            name="username"
            validators={{
              onChange: z.string().min(3, "Username must be at least 3 character(s)"),
              onChangeAsyncDebounceMs: 1000,
            }}
            children={(field) => (
              <FormItem field={field}>
                <Input
                  placeholder="ghosty"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)} />
              </FormItem>
            )}
          />
          <form.Field
            name="password"
            children={(field) => (
              <FormItem field={field}>
                <Input
                  placeholder="******"
                  type="password"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)} />
              </FormItem>
            )}
          />
          <Separator className="mt-2 mb-2" />
          <form.Field
            name="key"
            children={(field) => (
              <FormItem label="Encryption Key" field={field}>
                <Input
                  placeholder="******"
                  type="password"
                  id={field.name}
                  name={field.name}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)} />
              </FormItem>
            )}
          />
        </div>
        <div className="flex gap-2 mt-4">
          <Button type="submit">Login</Button>
        </div>
      </form>
    </div>
  )
}
