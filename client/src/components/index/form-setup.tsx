import { useNavigate } from '@tanstack/react-router'
import { z } from "zod"
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator"
import { Input } from "@/components/ui/input"
import { invoke } from '@tauri-apps/api/tauri'
import { useToast } from '@/components/ui/use-toast'
import { useForm } from "@tanstack/react-form"
import { zodValidator } from "@tanstack/zod-form-adapter"
import { FormItem } from "@/components/ui/tanstack-form"

export default function SetupForm() {
  const navigate = useNavigate();
  const { toast } = useToast();
  const form = useForm({
    defaultValues: {
      username: "",
      password: "",
      passwordConfirm: "",
      key: "",
      keyConfirm: ""
    },
    onSubmit: async ({ value }) => {
      invoke("user_create", value)
        .then(() => navigate({ to: '/agents' }))
        .catch(() =>
          toast({
            variant: "destructive",
            title: "Failed to setup",
            description: "Unable to create user, please review logs",
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
          <Separator className="mt-2 mb-2" />
          <form.Field
            name="password"
            validators={{
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: z.string().min(8, "Password must a minimum of 8 character(s)"),
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
          <form.Field
            name="passwordConfirm"
            validators={{
              onChangeListenTo: ['password'],
              onChangeAsyncDebounceMs: 800,
              onChangeAsync: ({ value, fieldApi }) => {
                if (value !== fieldApi.form.getFieldValue('password')) {
                  return 'Passwords do not match'
                }
                return undefined
              },
            }}
            children={(field) => (
              <FormItem label="Confirm Password" field={field}>
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
              onChangeAsync: z.string()
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
          <form.Field
            name="keyConfirm"
            validators={{
              onChangeListenTo: ['key'],
              onChangeAsyncDebounceMs: 800,
              onChangeAsync: ({ value, fieldApi }) => {
                if (value !== fieldApi.form.getFieldValue('key')) {
                  return 'Encryption keys do not match'
                }
                return undefined
              },
            }}
            children={(field) => (
              <FormItem label="Confirm Encryption Key" field={field}>
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
          <Button type="submit">Complete Setup</Button>
        </div>
      </form>
    </div>
  )
}
