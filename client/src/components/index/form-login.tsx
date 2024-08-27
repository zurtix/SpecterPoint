import { useNavigate } from '@tanstack/react-router'
import { useForm } from "@tanstack/react-form"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import { FormItem } from "@/components/ui/tanstack-form"
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
      invoke("login", { "creds": value }).then((_) =>
        navigate({
          to: "/targets"
        })
      ).catch((err) =>
        toast({
          variant: "destructive",
          title: "Failed to login",
          // description: "Review username, password, and encryption key",
          description: err
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
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.string().min(3, "Username must be at least 3 character(s)"),
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
            validators={{
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.string().min(1, "Password must be supplied"),
            }}
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
            validators={{
              onChange: z.string()
                .min(32, "Key must be a minimum of 32 character(s)")
                .max(32, "Key must be a maximum of 32 character(s)"),
              onChangeAsyncDebounceMs: 800
            }}
            children={(field) => (
              <FormItem label="Encryption Key" description="AES-256 GCM 32-Bit encryption key" field={field}>
                <Input
                  placeholder="********************************"
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
