import * as ToastPrimitive from '@radix-ui/react-toast';
import { forwardRef, type ComponentPropsWithoutRef, type ElementRef } from 'react';
import { cn } from '@/lib/cn';

export const ToastProvider = ToastPrimitive.Provider;

export const ToastViewport = forwardRef<
  ElementRef<typeof ToastPrimitive.Viewport>,
  ComponentPropsWithoutRef<typeof ToastPrimitive.Viewport>
>(({ className, ...props }, ref) => (
  <ToastPrimitive.Viewport
    ref={ref}
    className={cn(
      'fixed bottom-4 right-4 z-50 flex flex-col gap-2 outline-none',
      className,
    )}
    {...props}
  />
));
ToastViewport.displayName = 'ToastViewport';

export const Toast = forwardRef<
  ElementRef<typeof ToastPrimitive.Root>,
  ComponentPropsWithoutRef<typeof ToastPrimitive.Root>
>(({ className, ...props }, ref) => (
  <ToastPrimitive.Root
    ref={ref}
    className={cn(
      'rounded-tofa-md bg-brand text-on-brand px-4 py-2 font-mono text-xs shadow-lg',
      className,
    )}
    {...props}
  />
));
Toast.displayName = 'Toast';

export const ToastTitle  = ToastPrimitive.Title;
export const ToastAction = ToastPrimitive.Action;
export const ToastClose  = ToastPrimitive.Close;
