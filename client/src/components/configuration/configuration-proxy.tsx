"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"

const configureProxySchema = z.object({
  server: z.string(),
  username: z.string().optional(),
  password: z.string().optional(),
})

export default function ConfigureProxyForm() {
  const form = useForm<z.infer<typeof configureProxySchema>>({
    resolver: zodResolver(configureProxySchema),
  })

  function onSubmit(values: z.infer<typeof configureProxySchema>) {
    console.log(values)
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <div className="grid grid-cols-2 justify-between gap-4 p-2">
          <FormField
            control={form.control}
            name="server"
            render={({ field }) => (
              <FormItem className="col-span-2">
                <FormLabel>Server</FormLabel>
                <FormDescription>HTTP | HTTPS | SOCKS4 | SOCKS5</FormDescription>
                <FormControl>
                  <Input placeholder="https://example.com/" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
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
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input type="password" placeholder="******" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
        <div className="col-span-2 mt-4">
          <Button type="submit">Save</Button>
        </div>
      </form>
    </Form >
  )
}
