import {
  dotFileTest,
  lineAmountTest,
  operatorTest,
  throwErrorTest,
} from "./test_functions.js";
import { FileNotFoundError } from "../src/utils.js";

const TC_CSV = "test/rml-mapper-test-cases-csv"; // Test cases directory (csv)
const TC_JSON = "test/rml-mapper-test-cases-json"; // Test cases directory (json)

/*
 All RML-Mapper csv test-cases (https://github.com/kg-construct/rml-test-cases)
 Then converted using meamer-rs (https://github.com/s-minoo/meamer-rs)
 Currently, a lot are failing due to non-implemented operators or functions.

 */

describe("RML Mapper Tests CSV", () => {
  it(
    "RMLTC0000-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0000-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0000-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0001a-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0001a-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0001a-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0001b-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0001b-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0001b-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0002a-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0002a-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0002a-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0002b-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0002b-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0002b-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0003c-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0003c-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0003c-CSV/output.nq`,
    ),
  );
  it(
    "RMLTC0004a-CSV",
    dotFileTest(
      `${TC_CSV}/RMLTC0004a-CSV/plan.dot`,
      `${TC_CSV}/RMLTC0004a-CSV/output.nq`,
    ),
  );
    it(
      "RMLTC0005a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0005a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0005a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0006a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0006a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0006a-CSV/output.nq`,
        false
      ),
    );
    it(
      "RMLTC0007a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0007b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007b-CSV/output.nq`,
          false
      ),
    );
    it(
      "RMLTC0007c-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007c-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007c-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0007d-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007d-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007d-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0007e-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007e-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007e-CSV/output.nq`,
          false
      ),
    );
    it(
      "RMLTC0007f-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007f-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007f-CSV/output.nq`,
          false
      ),
    );
    it(
      "RMLTC0007g-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007g-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007g-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0007h-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0007h-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0007h-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0008a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0008a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0008a-CSV/output.nq`,
          false
      ),
    );
    it(
      "RMLTC0008b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0008b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0008b-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0008c-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0008c-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0008c-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0009a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0009a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0009a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0009b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0009b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0009b-CSV/output.nq`,
          false
      ),
    );
    it(
      "RMLTC0010a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0010a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0010a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0010b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0010b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0010b-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0010c-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0010c-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0010c-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0011b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0011b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0011b-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0012a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0012a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0012a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0012b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0012b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0012b-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0019a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0019a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0019a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0019b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0019b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0019b-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0020a-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0020a-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0020a-CSV/output.nq`,
      ),
    );
    it(
      "RMLTC0020b-CSV",
      dotFileTest(
        `${TC_CSV}/RMLTC0020b-CSV/plan.dot`,
        `${TC_CSV}/RMLTC0020b-CSV/output.nq`,
      ),
    );
});


describe("RML Mapper Tests JSON", () => {
    it('RMLTC0000-JSON', dotFileTest(`${TC_JSON}/RMLTC0000-JSON/plan.dot`, `${TC_JSON}/RMLTC0000-JSON/output.nq`));
    it('RMLTC0001a-JSON', dotFileTest(`${TC_JSON}/RMLTC0001a-JSON/plan.dot`, `${TC_JSON}/RMLTC0001a-JSON/output.nq`));
    it('RMLTC0001b-JSON', dotFileTest(`${TC_JSON}/RMLTC0001b-JSON/plan.dot`, `${TC_JSON}/RMLTC0001b-JSON/output.nq`));
    it('RMLTC0002a-JSON', dotFileTest(`${TC_JSON}/RMLTC0002a-JSON/plan.dot`, `${TC_JSON}/RMLTC0002a-JSON/output.nq`));
    it('RMLTC0002b-JSON', dotFileTest(`${TC_JSON}/RMLTC0002b-JSON/plan.dot`, `${TC_JSON}/RMLTC0002b-JSON/output.nq`));
    it('RMLTC0003c-JSON', dotFileTest(`${TC_JSON}/RMLTC0003c-JSON/plan.dot`, `${TC_JSON}/RMLTC0003c-JSON/output.nq`));
    it('RMLTC0004a-JSON', dotFileTest(`${TC_JSON}/RMLTC0004a-JSON/plan.dot`, `${TC_JSON}/RMLTC0004a-JSON/output.nq`));
    it('RMLTC0005a-JSON', dotFileTest(`${TC_JSON}/RMLTC0005a-JSON/plan.dot`, `${TC_JSON}/RMLTC0005a-JSON/output.nq`));
    it('RMLTC0006a-JSON', dotFileTest(`${TC_JSON}/RMLTC0006a-JSON/plan.dot`, `${TC_JSON}/RMLTC0006a-JSON/output.nq`, false));
    it('RMLTC0007a-JSON', dotFileTest(`${TC_JSON}/RMLTC0007a-JSON/plan.dot`, `${TC_JSON}/RMLTC0007a-JSON/output.nq`));
    it('RMLTC0007b-JSON', dotFileTest(`${TC_JSON}/RMLTC0007b-JSON/plan.dot`, `${TC_JSON}/RMLTC0007b-JSON/output.nq`, false));
    it('RMLTC0007c-JSON', dotFileTest(`${TC_JSON}/RMLTC0007c-JSON/plan.dot`, `${TC_JSON}/RMLTC0007c-JSON/output.nq`));
    it('RMLTC0007d-JSON', dotFileTest(`${TC_JSON}/RMLTC0007d-JSON/plan.dot`, `${TC_JSON}/RMLTC0007d-JSON/output.nq`));
    it('RMLTC0007e-JSON', dotFileTest(`${TC_JSON}/RMLTC0007e-JSON/plan.dot`, `${TC_JSON}/RMLTC0007e-JSON/output.nq`, false));
    it('RMLTC0007f-JSON', dotFileTest(`${TC_JSON}/RMLTC0007f-JSON/plan.dot`, `${TC_JSON}/RMLTC0007f-JSON/output.nq`, false));
    it('RMLTC0007g-JSON', dotFileTest(`${TC_JSON}/RMLTC0007g-JSON/plan.dot`, `${TC_JSON}/RMLTC0007g-JSON/output.nq`));
    it('RMLTC0007h-JSON', dotFileTest(`${TC_JSON}/RMLTC0007h-JSON/plan.dot`, `${TC_JSON}/RMLTC0007h-JSON/output.nq`));
    it('RMLTC0008a-JSON', dotFileTest(`${TC_JSON}/RMLTC0008a-JSON/plan.dot`, `${TC_JSON}/RMLTC0008a-JSON/output.nq`, false));
    it('RMLTC0008b-JSON', dotFileTest(`${TC_JSON}/RMLTC0008b-JSON/plan.dot`, `${TC_JSON}/RMLTC0008b-JSON/output.nq`));
    it('RMLTC0008c-JSON', dotFileTest(`${TC_JSON}/RMLTC0008c-JSON/plan.dot`, `${TC_JSON}/RMLTC0008c-JSON/output.nq`));
    it('RMLTC0009a-JSON', dotFileTest(`${TC_JSON}/RMLTC0009a-JSON/plan.dot`, `${TC_JSON}/RMLTC0009a-JSON/output.nq`));
    it('RMLTC0009b-JSON', dotFileTest(`${TC_JSON}/RMLTC0009b-JSON/plan.dot`, `${TC_JSON}/RMLTC0009b-JSON/output.nq`, false));
    it('RMLTC0010a-JSON', dotFileTest(`${TC_JSON}/RMLTC0010a-JSON/plan.dot`, `${TC_JSON}/RMLTC0010a-JSON/output.nq`));
    it('RMLTC0010b-JSON', dotFileTest(`${TC_JSON}/RMLTC0010b-JSON/plan.dot`, `${TC_JSON}/RMLTC0010b-JSON/output.nq`));
    it('RMLTC0010c-JSON', dotFileTest(`${TC_JSON}/RMLTC0010c-JSON/plan.dot`, `${TC_JSON}/RMLTC0010c-JSON/output.nq`));
    it('RMLTC0011b-JSON', dotFileTest(`${TC_JSON}/RMLTC0011b-JSON/plan.dot`, `${TC_JSON}/RMLTC0011b-JSON/output.nq`));
    it('RMLTC0012a-JSON', dotFileTest(`${TC_JSON}/RMLTC0012a-JSON/plan.dot`, `${TC_JSON}/RMLTC0012a-JSON/output.nq`));
    it('RMLTC0012b-JSON', dotFileTest(`${TC_JSON}/RMLTC0012b-JSON/plan.dot`, `${TC_JSON}/RMLTC0012b-JSON/output.nq`));
    it('RMLTC0019a-JSON', dotFileTest(`${TC_JSON}/RMLTC0019a-JSON/plan.dot`, `${TC_JSON}/RMLTC0019a-JSON/output.nq`));
    it('RMLTC0019b-JSON', dotFileTest(`${TC_JSON}/RMLTC0019b-JSON/plan.dot`, `${TC_JSON}/RMLTC0019b-JSON/output.nq`));
    it('RMLTC0020a-JSON', dotFileTest(`${TC_JSON}/RMLTC0020a-JSON/plan.dot`, `${TC_JSON}/RMLTC0020a-JSON/output.nq`));
    it('RMLTC0020b-JSON', dotFileTest(`${TC_JSON}/RMLTC0020b-JSON/plan.dot`, `${TC_JSON}/RMLTC0020b-JSON/output.nq`));
});

