import { ResponsiveDialog } from "@/components/ui/responsive-dialog"
import ListenerForm from "@/components/listener/form-listener"

export function CreateListener({ open, setOpen }:
  { open: boolean, setOpen: React.Dispatch<React.SetStateAction<boolean>> }) {
  return (
    <ResponsiveDialog
      isOpen={open}
      setIsOpen={setOpen}
      title="Create new Listener"
      description="Fill out the form below to create a new listener"
    >
      <ListenerForm setOpen={setOpen} />
    </ResponsiveDialog>
  )
}
