import * as React from "react"
import type { FieldApi } from "@tanstack/react-form"
import * as LabelPrimitive from "@radix-ui/react-label"
import { Label } from "@/components/ui/label"
import { cn } from "@/lib/utils"

interface FormItemProps extends React.HTMLAttributes<HTMLDivElement> {
  field: FieldApi<any, any, any, any>,
  label?: string,
  description?: string,
  children: React.ReactNode
}

export const FormItem = React.forwardRef<
  HTMLDivElement,
  FormItemProps
>(({ field, label, description, children, className }, ref) => {
  return (
    <div ref={ref} className={cn("flex flex-col gap-2", className)}>
      <FormLabel label={label} field={field} />
      {description &&
        <FormDescription>
          {description}
        </FormDescription>
      }
      {children}
      <FormMessage field={field} />
    </div>
  )
})


export const FormDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => {
  return (
    <p
      ref={ref}
      className={cn("text-sm text-muted-foreground", className)}
      {...props}
    />
  )
})

interface FormLabelProps extends React.ComponentPropsWithoutRef<typeof LabelPrimitive.Root> {
  field: FieldApi<any, any, any, any>,
  label?: string
}

export const FormLabel = React.forwardRef<
  React.ElementRef<typeof LabelPrimitive.Root>,
  FormLabelProps
>(
  ({ field, label, className, ...props }, ref) => {

    console.log(field.state.meta.errors)
    return (
      <Label
        ref={ref}
        className={cn(field.state.meta.errors.length > 0 && "text-destructive", "capitalize", className)}
        htmlFor={field.name}
        {...props}
      >
        {label ? label : field.name}
      </Label>
    )
  }
)


interface FormMessageProps extends React.HTMLAttributes<HTMLParagraphElement> {
  field: FieldApi<any, any, any, any>
}

export const FormMessage = React.forwardRef<
  HTMLParagraphElement,
  FormMessageProps
>(({ field, className, children, ...props }, ref) => {
  return (
    <p
      ref={ref}
      className={cn("text-sm font-medium text-destructive", className)}
      {...props}
    >
      {field.state.meta.errors ? (
        <em className="text-xs text-red-700" role="alert">{field.state.meta.errors.join(', ')}</em>
      ) : null}
    </p>
  )
})
