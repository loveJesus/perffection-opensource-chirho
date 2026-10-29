// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/reviewed-product-chirho.md.
// Bounded real-keyboard traversal shared by the customer-form browser journeys; never focus().
import type { Locator as LocatorChirho, Page as PageChirho } from "playwright-core";

export async function tab_to_chirho(page_chirho: PageChirho, target_chirho: LocatorChirho, backwards_chirho = false): Promise<number> {
  for (let count_chirho = 0; count_chirho <= 80; count_chirho++) {
    if (await target_chirho.evaluate(element_chirho => document.activeElement === element_chirho)) return count_chirho;
    await page_chirho.keyboard.press(backwards_chirho ? "Shift+Tab" : "Tab");
  }
  throw Error("Expected control cannot be reached by the bounded keyboard walk");
}

export async function control_and_label_visible_chirho(control_chirho: LocatorChirho): Promise<boolean> {
  return await control_chirho.evaluate(element_chirho => {
    const box_chirho = element_chirho.getBoundingClientRect();
    const label_chirho = (element_chirho as HTMLInputElement).labels?.[0];
    const label_box_chirho = label_chirho?.getBoundingClientRect();
    return box_chirho.top >= 0 && box_chirho.bottom <= innerHeight
      && document.elementFromPoint(box_chirho.left + box_chirho.width / 2, box_chirho.top + box_chirho.height / 2) === element_chirho
      && !!label_box_chirho && label_box_chirho.top >= 0 && label_box_chirho.bottom <= innerHeight;
  });
}
