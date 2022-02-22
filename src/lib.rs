#![recursion_limit = "1024"]

#[macro_use]
extern crate paste;
#[macro_use]
extern crate blanket;
extern crate horned_owl;

use horned_owl::model::*;

macro_rules! impl_traits {
    ($visit:ident, $($name:ident($type:ty),)*) => {
        paste! {
            #[blanket(default = "visit", derive(Mut, Box))]
            pub trait $visit<'ast> {
                $(
                    fn [<visit_ $name>] (&mut self, $name: &'ast $type);
                )*
            }
        }
    }
}

impl_traits! { Visit,
    annotated_axiom(AnnotatedAxiom),
    annotation(Annotation),
    annotation_assertion(AnnotationAssertion),
    annotation_property(AnnotationProperty),
    annotation_property_domain(AnnotationPropertyDomain),
    annotation_property_range(AnnotationPropertyRange),
    annotation_subject(AnnotationSubject),
    annotation_value(AnnotationValue),
    anonymous_individual(AnonymousIndividual),
    asymmetric_object_property(AsymmetricObjectProperty),
    axiom(Axiom),
    class(Class),
    class_assertion(ClassAssertion),
    class_expression(ClassExpression),
    data_property(DataProperty),
    data_property_assertion(DataPropertyAssertion),
    data_property_domain(DataPropertyDomain),
    data_property_range(DataPropertyRange),
    data_range(DataRange),
    datatype(Datatype),
    datatype_definition(DatatypeDefinition),
    declare_annotation_property(DeclareAnnotationProperty),
    declare_class(DeclareClass),
    declare_datatype(DeclareDatatype),
    declare_data_property(DeclareDataProperty),
    declare_named_individual(DeclareNamedIndividual),
    declare_object_property(DeclareObjectProperty),
    different_individuals(DifferentIndividuals),
    disjoint_classes(DisjointClasses),
    disjoint_data_properties(DisjointDataProperties),
    disjoint_object_properties(DisjointObjectProperties),
    disjoint_union(DisjointUnion),
    equivalent_classes(EquivalentClasses),
    equivalent_data_properties(EquivalentDataProperties),
    equivalent_object_properties(EquivalentObjectProperties),
    facet(Facet),
    facet_restriction(FacetRestriction),
    functional_data_property(FunctionalDataProperty),
    functional_object_property(FunctionalObjectProperty),
    has_key(HasKey),
    import(Import),
    individual(Individual),
    inverse_functional_object_property(InverseFunctionalObjectProperty),
    inverse_object_properties(InverseObjectProperties),
    iri(IRI),
    irreflexive_object_property(IrreflexiveObjectProperty),
    literal(Literal),
    named_individual(NamedIndividual),
    negative_data_property_assertion(NegativeDataPropertyAssertion),
    negative_object_property_assertion(NegativeObjectPropertyAssertion),
    object_property(ObjectProperty),
    object_property_assertion(ObjectPropertyAssertion),
    object_property_expression(ObjectPropertyExpression),
    object_property_domain(ObjectPropertyDomain),
    object_property_range(ObjectPropertyRange),
    ontology_annotation(OntologyAnnotation),
    property_expression(PropertyExpression),
    reflexive_object_property(ReflexiveObjectProperty),
    same_individual(SameIndividual),
    sub_class_of(SubClassOf),
    sub_annotation_property_of(SubAnnotationPropertyOf),
    sub_data_property_of(SubDataPropertyOf),
    sub_object_property_expression(SubObjectPropertyExpression),
    sub_object_property_of(SubObjectPropertyOf),
    symmetric_object_property(SymmetricObjectProperty),
    transitive_object_property(TransitiveObjectProperty),
}

macro_rules! impl_default {
    ( $visitor:ident, $($name:ident($type:ty) => $code:expr,)* ) => {
        paste! {
            pub mod visit {

                use super::*;

                $(
                    #[allow(unused_variables)]
                    pub fn [<visit_ $name>] <'ast, V: Visit<'ast> + ?Sized>($visitor: &mut V, $name: &'ast $type) {
                        $code
                    }
                )*
            }
        }
    }
}

impl_default! { visitor,

    annotated_axiom(AnnotatedAxiom) => {
        annotated_axiom.ann.iter().for_each(|a| visitor.visit_annotation(a));
        visitor.visit_axiom(&annotated_axiom.axiom);
    },

    annotation(Annotation) => {
        visitor.visit_annotation_property(&annotation.ap);
        visitor.visit_annotation_value(&annotation.av);
    },

    annotation_assertion(AnnotationAssertion) => {
        visitor.visit_annotation_subject(&annotation_assertion.subject);
        visitor.visit_annotation(&annotation_assertion.ann);
    },

    annotation_property(AnnotationProperty) => {
        visitor.visit_iri(&annotation_property.0);
    },

    annotation_property_domain(AnnotationPropertyDomain) => {
        visitor.visit_annotation_property(&annotation_property_domain.ap);
        visitor.visit_iri(&annotation_property_domain.iri);
    },

    annotation_property_range(AnnotationPropertyRange) => {
        visitor.visit_annotation_property(&annotation_property_range.ap);
        visitor.visit_iri(&annotation_property_range.iri);
    },

    annotation_subject(AnnotationSubject) => {
        use self::AnnotationSubject::*;
        match &annotation_subject {
            IRI(iri) => visitor.visit_iri(iri),
            AnonymousIndividual(ai) => visitor.visit_anonymous_individual(ai),
        }
    },

    annotation_value(AnnotationValue) => {
        match annotation_value {
            AnnotationValue::Literal(lit) => visitor.visit_literal(&lit),
            AnnotationValue::IRI(iri) => visitor.visit_iri(&iri),
        }
    },

    anonymous_individual(AnonymousIndividual) => {},

    asymmetric_object_property(AsymmetricObjectProperty) => {
        visitor.visit_object_property_expression(&asymmetric_object_property.0);
    },

    axiom(Axiom) => {
        use self::Axiom::*;
        match &axiom {
            OntologyAnnotation(oa) => visitor.visit_ontology_annotation(oa),
            Import(import) => visitor.visit_import(import),
            DeclareClass(dc) => visitor.visit_declare_class(dc),
            DeclareObjectProperty(dop) => visitor.visit_declare_object_property(dop),
            DeclareAnnotationProperty(dap) => visitor.visit_declare_annotation_property(dap),
            DeclareDataProperty(ddp) => visitor.visit_declare_data_property(ddp),
            DeclareNamedIndividual(dni) => visitor.visit_declare_named_individual(dni),
            DeclareDatatype(ddt) => visitor.visit_declare_datatype(ddt),
            SubClassOf(sco) => visitor.visit_sub_class_of(sco),
            EquivalentClasses(ec) => visitor.visit_equivalent_classes(ec),
            DisjointClasses(dc) => visitor.visit_disjoint_classes(dc),
            DisjointUnion(du) => visitor.visit_disjoint_union(du),
            SubObjectPropertyOf(sopo) => visitor.visit_sub_object_property_of(sopo),
            EquivalentObjectProperties(eop) => visitor.visit_equivalent_object_properties(eop),
            DisjointObjectProperties(dop) => visitor.visit_disjoint_object_properties(dop),
            InverseObjectProperties(iop) => visitor.visit_inverse_object_properties(iop),
            ObjectPropertyDomain(opd) => visitor.visit_object_property_domain(opd),
            ObjectPropertyRange(opr) => visitor.visit_object_property_range(opr),
            FunctionalObjectProperty(fop) => visitor.visit_functional_object_property(fop),
            InverseFunctionalObjectProperty(ifop) => visitor.visit_inverse_functional_object_property(ifop),
            ReflexiveObjectProperty(rop) => visitor.visit_reflexive_object_property(rop),
            IrreflexiveObjectProperty(iop) => visitor.visit_irreflexive_object_property(iop),
            SymmetricObjectProperty(sop) => visitor.visit_symmetric_object_property(sop),
            AsymmetricObjectProperty(aop) => visitor.visit_asymmetric_object_property(aop),
            TransitiveObjectProperty(top) => visitor.visit_transitive_object_property(top),
            SubDataPropertyOf(sdpo) => visitor.visit_sub_data_property_of(sdpo),
            EquivalentDataProperties(edp) => visitor.visit_equivalent_data_properties(edp),
            DisjointDataProperties(ddp) => visitor.visit_disjoint_data_properties(ddp),
            DataPropertyDomain(dpd) => visitor.visit_data_property_domain(dpd),
            DataPropertyRange(dpr) => visitor.visit_data_property_range(dpr),
            FunctionalDataProperty(fdp) => visitor.visit_functional_data_property(fdp),
            DatatypeDefinition(dd) => visitor.visit_datatype_definition(dd),
            HasKey(hk) => visitor.visit_has_key(hk),
            SameIndividual(si) => visitor.visit_same_individual(si),
            DifferentIndividuals(di) => visitor.visit_different_individuals(di),
            ClassAssertion(ca) => visitor.visit_class_assertion(ca),
            ObjectPropertyAssertion(opa) => visitor.visit_object_property_assertion(opa),
            NegativeObjectPropertyAssertion(nopa) => visitor.visit_negative_object_property_assertion(nopa),
            DataPropertyAssertion(dpa) => visitor.visit_data_property_assertion(dpa),
            NegativeDataPropertyAssertion(ndpa) => visitor.visit_negative_data_property_assertion(ndpa),
            AnnotationAssertion(aa) => visitor.visit_annotation_assertion(aa),
            SubAnnotationPropertyOf(sapo) => visitor.visit_sub_annotation_property_of(sapo),
            AnnotationPropertyDomain(apd) => visitor.visit_annotation_property_domain(apd),
            AnnotationPropertyRange(apr) => visitor.visit_annotation_property_range(apr),
        }
    },

    class(Class) => {
        visitor.visit_iri(&class.0);
    },

    class_assertion(ClassAssertion) => {
        visitor.visit_class_expression(&class_assertion.ce);
        visitor.visit_individual(&class_assertion.i);
    },

    class_expression(ClassExpression) => {
        use self::ClassExpression::*;
        match &class_expression {
            Class(c) => visitor.visit_class(c),
            ObjectIntersectionOf(ces) => ces.iter().for_each(|ce| visitor.visit_class_expression(ce)),
            ObjectUnionOf(ces) => ces.iter().for_each(|ce| visitor.visit_class_expression(ce)),
            ObjectComplementOf(ce) => visitor.visit_class_expression(ce),
            ObjectOneOf(is) => is.iter().for_each(|i| visitor.visit_individual(i)),
            ObjectHasSelf(ope) => visitor.visit_object_property_expression(ope),
            ObjectSomeValuesFrom { ope, bce }      |
            ObjectAllValuesFrom { ope, bce }       |
            ObjectMinCardinality { ope, bce, .. }   |
            ObjectMaxCardinality { ope, bce, .. }   |
            ObjectExactCardinality { ope, bce, .. } => {
                visitor.visit_object_property_expression(ope);
                visitor.visit_class_expression(bce);
            },
            ObjectHasValue { ope, i } => {
                visitor.visit_object_property_expression(ope);
                visitor.visit_individual(i);
            },
            DataSomeValuesFrom { dp, dr }       |
            DataAllValuesFrom { dp, dr }       |
            DataMinCardinality { dp, dr, .. }   |
            DataMaxCardinality { dp, dr, .. }   |
            DataExactCardinality { dp, dr, .. } => {
                visitor.visit_data_property(dp);
                visitor.visit_data_range(dr);
            },
            DataHasValue { dp, l } => {
                visitor.visit_data_property(dp);
                visitor.visit_literal(l);
            },
        }
    },

    data_property(DataProperty) => {
        visitor.visit_iri(&data_property.0);
    },

    data_property_assertion(DataPropertyAssertion) => {
        visitor.visit_data_property(&data_property_assertion.dp);
        visitor.visit_individual(&data_property_assertion.from);
        visitor.visit_literal(&data_property_assertion.to);
    },

    data_property_domain(DataPropertyDomain) => {
        visitor.visit_data_property(&data_property_domain.dp);
        visitor.visit_class_expression(&data_property_domain.ce);
    },

    data_property_range(DataPropertyRange) => {
        visitor.visit_data_property(&data_property_range.dp);
        visitor.visit_data_range(&data_property_range.dr);
    },

    data_range(DataRange) => {
        use self::DataRange::*;
        match &data_range {
            Datatype(dt) => visitor.visit_datatype(dt),
            DataComplementOf(dr) => visitor.visit_data_range(dr),
            DataIntersectionOf(drs) => drs.iter().for_each(|dr| visitor.visit_data_range(dr)),
            DataUnionOf(drs) => drs.iter().for_each(|dr| visitor.visit_data_range(dr)),
            DataOneOf(lits) => lits.iter().for_each(|lit| visitor.visit_literal(lit)),
            DatatypeRestriction(dt, frs) => {
                visitor.visit_datatype(dt);
                frs.iter().for_each(|fr| visitor.visit_facet_restriction(fr));
            }
        }
    },

    datatype(Datatype) => {
        visitor.visit_iri(&datatype.0);
    },

    datatype_definition(DatatypeDefinition) => {
        visitor.visit_datatype(&datatype_definition.kind);
        visitor.visit_data_range(&datatype_definition.range);
    },

    declare_annotation_property(DeclareAnnotationProperty) => {
        visitor.visit_annotation_property(&declare_annotation_property.0);
    },

    declare_class(DeclareClass) => {
        visitor.visit_class(&declare_class.0);
    },

    declare_datatype(DeclareDatatype) => {
        visitor.visit_datatype(&declare_datatype.0);
    },

    declare_data_property(DeclareDataProperty) => {
        visitor.visit_data_property(&declare_data_property.0);
    },

    declare_named_individual(DeclareNamedIndividual) => {
        visitor.visit_named_individual(&declare_named_individual.0);
    },

    declare_object_property(DeclareObjectProperty) => {
        visitor.visit_object_property(&declare_object_property.0);
    },

    different_individuals(DifferentIndividuals) => {
        different_individuals.0.iter().for_each(|i| visitor.visit_individual(i));
    },

    disjoint_classes(DisjointClasses) => {
        disjoint_classes.0.iter().for_each(|ce| visitor.visit_class_expression(ce));
    },

    disjoint_data_properties(DisjointDataProperties) => {
        disjoint_data_properties.0.iter().for_each(|dp| visitor.visit_data_property(dp));
    },

    disjoint_object_properties(DisjointObjectProperties) => {
        disjoint_object_properties.0.iter().for_each(|ope| visitor.visit_object_property_expression(ope));
    },

    disjoint_union(DisjointUnion) => {
        visitor.visit_class(&disjoint_union.0);
        disjoint_union.1.iter().for_each(|ce| visitor.visit_class_expression(ce));
    },

    equivalent_classes(EquivalentClasses) => {
        equivalent_classes.0.iter().for_each(|ce| visitor.visit_class_expression(ce));
    },

    equivalent_data_properties(EquivalentDataProperties) => {
        equivalent_data_properties.0.iter().for_each(|dp| visitor.visit_data_property(dp));
    },

    equivalent_object_properties(EquivalentObjectProperties) => {
        equivalent_object_properties.0.iter().for_each(|ope| visitor.visit_object_property_expression(ope));
    },

    facet(Facet) => {},

    facet_restriction(FacetRestriction) => {
        visitor.visit_facet(&facet_restriction.f);
        visitor.visit_literal(&facet_restriction.l);
    },

    functional_data_property(FunctionalDataProperty) => {
        visitor.visit_data_property(&functional_data_property.0);
    },

    functional_object_property(FunctionalObjectProperty) => {
        visitor.visit_object_property_expression(&functional_object_property.0);
    },

    has_key(HasKey) => {
        visitor.visit_class_expression(&has_key.ce);
        has_key.vpe.iter().for_each(|pe| visitor.visit_property_expression(pe));
    },

    import(Import) => {
        visitor.visit_iri(&import.0);
    },

    individual(Individual) => {
        use self::Individual::*;
        match &individual {
            Anonymous(a) => visitor.visit_anonymous_individual(a),
            Named(n) => visitor.visit_named_individual(n),
        }
    },

    inverse_functional_object_property(InverseFunctionalObjectProperty) => {
        visitor.visit_object_property_expression(&inverse_functional_object_property.0);
    },

    inverse_object_properties(InverseObjectProperties) => {
        visitor.visit_object_property(&inverse_object_properties.0);
        visitor.visit_object_property(&inverse_object_properties.1);
    },

    irreflexive_object_property(IrreflexiveObjectProperty) => {
        visitor.visit_object_property_expression(&irreflexive_object_property.0);
    },

    iri(IRI) => {},

    literal(Literal) => {},

    named_individual(NamedIndividual) => {
        visitor.visit_iri(&named_individual.0);
    },

    negative_data_property_assertion(NegativeDataPropertyAssertion) => {
        visitor.visit_data_property(&negative_data_property_assertion.dp);
        visitor.visit_individual(&negative_data_property_assertion.from);
        visitor.visit_literal(&negative_data_property_assertion.to);
    },

    negative_object_property_assertion(NegativeObjectPropertyAssertion) => {
        visitor.visit_object_property_expression(&negative_object_property_assertion.ope);
        visitor.visit_individual(&negative_object_property_assertion.from);
        visitor.visit_individual(&negative_object_property_assertion.to);
    },

    object_property(ObjectProperty) => {
        visitor.visit_iri(&object_property.0);
    },

    object_property_assertion(ObjectPropertyAssertion) => {
        visitor.visit_object_property_expression(&object_property_assertion.ope);
        visitor.visit_individual(&object_property_assertion.from);
        visitor.visit_individual(&object_property_assertion.to);
    },

    object_property_domain(ObjectPropertyDomain) => {
        visitor.visit_object_property_expression(&object_property_domain.ope);
        visitor.visit_class_expression(&object_property_domain.ce);
    },

    object_property_expression(ObjectPropertyExpression) => {
        use self::ObjectPropertyExpression::*;
        match &object_property_expression {
            ObjectProperty(op) => visitor.visit_object_property(op),
            InverseObjectProperty(op) => visitor.visit_object_property(op),
        }
    },

    object_property_range(ObjectPropertyRange) => {
        visitor.visit_object_property_expression(&object_property_range.ope);
        visitor.visit_class_expression(&object_property_range.ce);
    },

    ontology_annotation(OntologyAnnotation) => {
        visitor.visit_annotation(&ontology_annotation.0);
    },

    property_expression(PropertyExpression) => {
        use self::PropertyExpression::*;
        match &property_expression {
            ObjectPropertyExpression(ope) => visitor.visit_object_property_expression(ope),
            DataProperty(dp) => visitor.visit_data_property(dp),
            AnnotationProperty(ap) => visitor.visit_annotation_property(ap),
        }
    },

    reflexive_object_property(ReflexiveObjectProperty) => {
        visitor.visit_object_property_expression(&reflexive_object_property.0);
    },

    same_individual(SameIndividual) => {
        same_individual.0.iter().for_each(|i| visitor.visit_individual(i));
    },

    sub_annotation_property_of(SubAnnotationPropertyOf) => {
        visitor.visit_annotation_property(&sub_annotation_property_of.sup);
        visitor.visit_annotation_property(&sub_annotation_property_of.sub);
    },

    sub_class_of(SubClassOf) => {
        visitor.visit_class_expression(&sub_class_of.sup);
        visitor.visit_class_expression(&sub_class_of.sub);
    },

    sub_data_property_of(SubDataPropertyOf) => {
        visitor.visit_data_property(&sub_data_property_of.sup);
        visitor.visit_data_property(&sub_data_property_of.sub);
    },

    sub_object_property_expression(SubObjectPropertyExpression) => {
        use self::SubObjectPropertyExpression::*;
        match &sub_object_property_expression {
            ObjectPropertyExpression(ope) => visitor.visit_object_property_expression(ope),
            ObjectPropertyChain(opes) => opes.iter().for_each(|ope| visitor.visit_object_property_expression(ope)),
        }
    },

    sub_object_property_of(SubObjectPropertyOf) => {
        visitor.visit_object_property_expression(&sub_object_property_of.sup);
        visitor.visit_sub_object_property_expression(&sub_object_property_of.sub);
    },

    symmetric_object_property(SymmetricObjectProperty) => {
        visitor.visit_object_property_expression(&symmetric_object_property.0);
    },

    transitive_object_property(TransitiveObjectProperty) => {
        visitor.visit_object_property_expression(&transitive_object_property.0);
    },
}
