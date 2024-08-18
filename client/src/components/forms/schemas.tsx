import { z } from "zod"

export function getDefaults<Schema extends z.AnyZodObject>(schema: Schema) {
  return Object.fromEntries(
    Object.entries(schema.shape).map(([key, value]) => {
      if (value instanceof z.ZodDefault) return [key, value._def.defaultValue()]
      return [key, undefined]
    })
  )
}

export const manualServerSchema = z.object({
  name: z.string(),
  host: z.string(),
  port: z.string(),
  username: z.string(),
  password: z.string().min(8),
})

export type ManualServerSchema = z.infer<typeof manualServerSchema>


