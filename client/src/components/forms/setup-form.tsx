import { useNavigate } from '@tanstack/react-router'
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormMessage,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { Separator } from "@/components/ui/separator"
import { Input } from "@/components/ui/input"
import { HelpCircle } from "lucide-react"
import { invoke } from '@tauri-apps/api/tauri'
import { useToast } from '@/components/ui/use-toast'

const signupSchema = z.object({
  username: z.string().min(6),
  password: z.string().min(8),
  passwordConfirm: z.string().min(8),
  key: z.string().min(32).max(32),
  keyConfirm: z.string().min(32).max(32),
}).superRefine(({ passwordConfirm, password, keyConfirm, key }, ctx) => {
  if (passwordConfirm !== password) {
    ctx.addIssue({
      code: "custom",
      message: "The passwords did not match",
      path: ['passwordConfirm']
    });
  }
  if (keyConfirm !== key) {
    ctx.addIssue({
      code: "custom",
      message: "The encryption keys did not match",
      path: ['keyConfirm']
    });
  }
});

export default function SetupForm() {

  const navigate = useNavigate();
  const { toast } = useToast();

  const form = useForm<z.infer<typeof signupSchema>>({
    resolver: zodResolver(signupSchema),
  })

  function onSubmit(values: z.infer<typeof signupSchema>) {
    // TODO: Register the setup via invoke tauri app to sqlite
    invoke("create_user", values)
      .then((_) => navigate({ to: '/targets' }))
      .catch((err) =>
        toast({
          variant: "destructive",
          title: err,
          description: "Review username, password, and encryption key",
        })

      )

    console.log(values)
    navigate({
      to: '/'
    })
  }

  return (
    <div className="w-full">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)}>
          <div className="flex flex-col gap-2 w-full">
            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold">Username</FormLabel>
                  <FormControl>
                    <Input placeholder="ghosty" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold">Password</FormLabel>
                  <FormControl>
                    <Input type="password" placeholder="******" {...field} />
                  </FormControl>
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="passwordConfirm"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold">Confirm password</FormLabel>
                  <FormControl>
                    <Input type="password" placeholder="******" {...field} />
                  </FormControl>
                </FormItem>
              )}
            />
            <Separator className="mt-2 mb-2" />
            <FormField
              control={form.control}
              name="key"
              render={({ field }) => (
                <FormItem>
                  <div className="flex justify-between">
                    <FormLabel className="font-bold">Encryption Key</FormLabel>
                    <Popover>
                      <PopoverTrigger asChild>
                        <HelpCircle width={15} height={15} />
                      </PopoverTrigger>
                      <PopoverContent>
                        <p>AES-256 CBC 32-Bit encryption key</p>
                      </PopoverContent>
                    </Popover>

                  </div>
                  <FormControl>
                    <Input type="password" placeholder="******" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="keyConfirm"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold">Confirm encryption key</FormLabel>
                  <FormControl>
                    <Input type="password" placeholder="******" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

          </div>
          <div className="flex gap-2 mt-4">
            <Button type="submit">Complete Setup</Button>
          </div>
        </form>
      </Form>
    </div>
  )
}
