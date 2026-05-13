import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from './ui/accordion';

export interface FaqEntry { q: string; a: string }
interface Props { entries: FaqEntry[] }

export default function FAQ({ entries }: Props) {
  return (
    <Accordion type="single" collapsible className="w-full">
      {entries.map((e, i) => (
        <AccordionItem key={i} value={`item-${i}`}>
          <AccordionTrigger data-umami-event={`faq-open-${i + 1}`}>{e.q}</AccordionTrigger>
          <AccordionContent>{e.a}</AccordionContent>
        </AccordionItem>
      ))}
    </Accordion>
  );
}
