import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import {
  CheckIcon,
  XCircle,
  ChevronDown,
  XIcon,
  WandSparkles,
} from "lucide-react";

import { cn } from "@/lib/utils";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { PlusIcon } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area"

const multiBadgeVariants = cva(
  "m-1 transition ease-in-out delay-150 hover:-translate-y-1 hover:scale-110 duration-300",
  {
    variants: {
      variant: {
        default:
          "border-foreground/10 text-foreground bg-card hover:bg-card/80",
        secondary:
          "border-foreground/10 bg-secondary text-secondary-foreground hover:bg-secondary/80",
        destructive:
          "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
        inverted: "inverted",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
);

interface MultiBadgeProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  onValueChange: (value: string[]) => void;
  placeholder?: string;
  animation?: number;
  maxCount?: number;
  modalPopover?: boolean;
  asChild?: boolean;
  className?: string;
}

export const MultiBadge = React.forwardRef<
  HTMLButtonElement,
  MultiBadgeProps
>(
  (
    {
      onValueChange,
      placeholder = "Add New",
      modalPopover = false,
      asChild = false,
      className,
      ...props
    },
    ref
  ) => {
    const [selectedValues, setSelectedValues] =
      React.useState<string[]>([]);
    const [newValue, setNewValue] = React.useState<string>("");

    function handleChange(e: React.FormEvent<HTMLInputElement>) {
      setNewValue(e.currentTarget.value);
    }

    function handleClick() {
      if (newValue && !selectedValues.includes(newValue)) {
        setSelectedValues([...selectedValues, newValue]);
        onValueChange(selectedValues);
      }

      setNewValue("");
    }

    React.useEffect(() => {
      if (JSON.stringify(selectedValues) !== JSON.stringify([])) {
        setSelectedValues(selectedValues);
      }
    }, [selectedValues]);

    return (
      <>
        <div>
          <div className="flex">
            <Input type="text" placeholder="index.html" onChange={handleChange} value={newValue} />
            <Button type="button" className="bg-secondary ml-2" onClick={handleClick}><PlusIcon /></Button>
          </div>
          <ScrollArea className="h-32 mt-6">
            {
              selectedValues.map((v) => {
                return (
                  <Badge className="m-1 text-sm" key={v}>{v}</Badge>
                )
              })
            }
          </ScrollArea>
        </div>
      </>
    );
  }
);

MultiBadge.displayName = "MultiBadge";
